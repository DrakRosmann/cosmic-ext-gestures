# Gestos do touchpad para o COSMIC

Os gestos do GNOME no COSMIC, e um app para escolher o que cada gesto faz.

O COSMIC 1.8 só troca de espaço de trabalho com quatro dedos para os lados; três dedos
não fazem nada (nem chegam aos apps), e não há como configurar. Isto são duas partes:

- **`compositor/`**: patches para o `cosmic-comp` (os gestos, lidos de uma configuração;
  as animações das janelas e do shell como as do GNOME; e uma visão geral mais leve) e
  um para o `cosmic-workspaces` (a visão geral), com scripts para compilar, instalar e
  voltar ao original;
- **o app "Gestos do touchpad"**: escolhe o que cada gesto faz, e mostra se o compositor
  em uso tem o patch.

![](assets/screenshot.png)

## O que dá para fazer

Para três e quatro dedos, deslizando para cima, para baixo e para os lados:

| Ação | |
| --- | --- |
| Trocar de espaço de trabalho | acompanhando os dedos, na direção em que os espaços estão (horizontal ou vertical) |
| Visão geral, depois apps | como no GNOME: abre os espaços de trabalho; de novo, a biblioteca de apps |
| Espaços de trabalho, Biblioteca de apps, Iniciador | abrem ou fecham cada um |
| Voltar | fecha a biblioteca de apps, o iniciador ou os espaços de trabalho |
| Executar comando | qualquer comando |
| Nada | com todas as direções em Nada, o gesto vai para o app sob o cursor |

O padrão é o do GNOME: três ou quatro dedos para cima abrem a visão geral e depois os
apps, para baixo voltam, para os lados trocam de espaço. "Padrão do COSMIC" volta ao
original (só quatro dedos para os lados).

As ações que abrem algo esperam os dedos andarem um pouco (60 px), para um toque à toa não
abrir nada. A visão geral do COSMIC é um programa à parte (`cosmic-workspaces`): ela abre
quando o gesto passa desse ponto, sem crescer junto com os dedos como no GNOME.

## Animações

Como no GNOME (os tempos e as curvas são os do `gnome-shell`):

| | COSMIC 1.8 | Com o patch |
| --- | --- | --- |
| Abrir janela | aparece de uma vez | cresce do meio da borda de baixo e aparece aos poucos (150 ms) |
| Fechar janela | some de uma vez | encolhe um pouco e some aos poucos (150 ms) |
| Minimizar e restaurar | 320 ms, curva simétrica | 400 ms, rápido no começo e devagar no fim |
| Maximizar e encaixar | 200 ms, curva simétrica | 250 ms, desacelerando |
| Trocar de espaço pelo teclado | 200 ms, curva simétrica | 250 ms, desacelerando |
| Biblioteca de apps | aparece e some de uma vez | cresce do centro e aparece aos poucos (250 ms); ao fechar, encolhe e some (200 ms) |
| Iniciador | aparece e some de uma vez | cresce de cima e aparece aos poucos (200 ms); ao fechar, o contrário (150 ms) |
| Visão geral (espaços de trabalho) | aparece e some de uma vez, e as janelas somem junto | assenta, de um pouco maior, e aparece aos poucos enquanto as janelas somem por baixo (250 ms); ao fechar, o contrário |
| Popups dos applets do painel | aparecem e somem de uma vez | crescem a partir do painel e aparecem aos poucos (150 ms); ao fechar, o contrário |

Abrir vale para janelas flutuantes (o padrão do COSMIC); no modo lado a lado as janelas
continuam entrando como antes. Para fechar, o compositor guarda uma imagem da janela no
momento em que o app a fecha e é ela que some; o mesmo vale para a biblioteca de apps,
o iniciador, a visão geral e os popups, que os apps destroem ao fechar.

O desfoque do fundo (da biblioteca de apps, do iniciador, dos popups e das janelas
translúcidas) aparece e some junto com o que está na frente dele, em vez de surgir
inteiro antes do conteúdo e sumir de uma vez. Os apps do shell do COSMIC mostram primeiro
um quadro vazio e o conteúdo um ou dois quadros depois: a animação começa no segundo
quadro, senão ela podia passar inteira sobre o quadro vazio. E ao fechar eles desligam o
desfoque e os cantos e desenham um último quadro (o da biblioteca de apps, sem os
ícones) que a tela nunca chega a mostrar: o que fecha é o que estava na tela antes dele.

### Visão geral sem engasgos

Com a visão geral aberta, o `cosmic-workspaces` mostra miniaturas de cada espaço de
trabalho e de cada janela, que o compositor copia para ele. No COSMIC 1.8 isso trava a
visão geral (e o que se anima com ela) por dois motivos:

- o compositor copiava cada espaço e cada janela o tempo todo, mudassem ou não: ao
  terminar uma cópia, a área que o app pedia para atualizar voltava como "mudou", e ele
  pedia de novo, umas 70 vezes por segundo cada uma, mesmo em espaços vazios. Com o patch,
  uma cópia só sai quando aquilo muda, no máximo 30 vezes por segundo (são miniaturas), e
  informa só o que mudou;
- em GPUs Intel recentes (Tiger, Meteor, Arrow e Lunar Lake) o `cosmic-workspaces`
  recebia as cópias pela memória comum em vez da GPU (um contorno para um defeito de
  imagem relatado em 2025), e o compositor relia cada uma da GPU e copiava megabytes
  por quadro. O patch dele usa a GPU em todas, e só descarta as miniaturas depois de
  fechar, para elas aparecerem enquanto a visão geral some.

Num Meteor Lake, com a visão geral aberta o compositor perdia 53 de cada 88 quadros
(uns 24 por segundo); com os patches, nenhum depois de aberta.

## Instalar

Funciona no Fedora, no Debian e derivados (Ubuntu, Pop!_OS, Mint…) e no Arch e
derivados (Manjaro, EndeavourOS…), com o COSMIC 1.8.0 instalado pelo sistema. Os
scripts pedem `sudo` para instalar o que faltar e para trocar o `/usr/bin/cosmic-comp`.

O compositor e a visão geral:

```sh
compositor/install.sh
```

Ele descobre as versões do `cosmic-comp` e do `cosmic-workspaces` instaladas (pelo `rpm`,
`dpkg` ou `pacman`), baixa o código delas, aplica os patches, instala o que faltar para
compilar (`deps.sh`, com `dnf`, `apt` ou `pacman`), compila, guarda os originais como
`cosmic-comp.orig` e `cosmic-workspaces.orig` e põe os novos no lugar. Saia da sessão e
entre de novo.

O `cosmic-comp` pede um Rust recente (1.93 para o 1.8.0), mais novo que o de várias
distribuições (Debian, Ubuntu e Pop!_OS LTS). Se o do sistema for antigo, o script avisa:
instale o Rust pelo [rustup](https://rustup.rs) e rode de novo.

Os patches são para o `cosmic-comp` 1.8.0. Com outra versão, o script avisa e para: o
patch precisa ser atualizado para ela.

Se a sessão não abrir: Ctrl+Alt+F3, entre, e rode `sudo mv /usr/bin/cosmic-comp.orig
/usr/bin/cosmic-comp`, ou reinstale o pacote:

| | |
| --- | --- |
| Fedora | `sudo dnf reinstall cosmic-comp` |
| Debian, Ubuntu, Pop!_OS | `sudo apt install --reinstall cosmic-comp` |
| Arch | `sudo pacman -S cosmic-comp` |

`compositor/restore.sh` põe os dois originais de volta de dentro da sessão. Se as
miniaturas da visão geral piscarem ou aparecerem cortadas (o defeito que o contorno da
Intel evitava), `sudo mv /usr/bin/cosmic-workspaces.orig /usr/bin/cosmic-workspaces` volta
só a visão geral.

Uma atualização do `cosmic-comp` ou do `cosmic-workspaces` pelo sistema os troca de volta
pelos originais: rode `compositor/install.sh` de novo (para uma versão nova, os patches
precisam ser atualizados; o app avisa quando o compositor em uso não tem os gestos).

O app:

```sh
./install.sh            # em ~/.local
```

## Como funciona

A configuração fica em `~/.config/cosmic/io.github.DrakRosmann.CosmicGestures/v1/bindings`
(RON), escrita pelo app e lida pelo compositor, que a recarrega ao mudar. O patch
(`compositor/cosmic-comp-1.8.0-gestures.patch`) troca o trecho em que o `cosmic-comp`
decidia os gestos de quatro dedos (`src/input/mod.rs`) pela leitura dessa configuração;
a troca de espaço continua sendo a do COSMIC, que acompanha os dedos, e as outras ações
usam os mesmos comandos dos atalhos de teclado (`Super+W`, `Super+A`…).

## In English

GNOME's touchpad gestures for COSMIC: a patch for `cosmic-comp` 1.8.0 that reads which
three- and four-finger swipes do what (switch workspace following the fingers, overview
then app library, app library, launcher, go back, run a command), scripts to build,
install and restore it, and an app to set the gestures. Another patch makes window
animations GNOME's: windows open growing from their bottom middle and close shrinking
and fading, and minimizing, maximizing and switching workspaces use GNOME's timing and
curves. A third one animates the shell's layers like GNOME's: the app library,
the launcher, the workspaces overview (the windows fading out under it) and the
panel's popups fade in growing into place, and close the other way, their background blur along with them; they begin with
their second frame, as COSMIC's apps show an empty one first, and close as they were
before the last frame those apps draw as they go (without their blur and corners, and
the app library's without its icons). And the overview stops stuttering: `cosmic-comp`
copied every workspace and window to it all the time, changed or not (the damage the
client asked to have redrawn came back as damage, so it asked again, about 70 times a
second each), and now copies each only when it changes, at most 30 times a second;
`cosmic-workspaces` is patched to copy with the GPU on Intel Tiger/Meteor/Arrow/Lunar
Lake too, instead of through shared memory, which the compositor read back and copied
every frame. `compositor/install.sh` swaps in both; `compositor/restore.sh` puts the
originals back.

It works on Fedora, Debian and its derivatives (Ubuntu, Pop!_OS, Mint…) and Arch and its
derivatives: `compositor/install.sh` finds the installed `cosmic-comp` version with `rpm`,
`dpkg` or `pacman`, and `deps.sh` installs what building it needs with `dnf`, `apt` or
`pacman`. `cosmic-comp` 1.8.0 needs Rust 1.93 or newer; where the distribution's is older,
install it with [rustup](https://rustup.rs). Then `./install.sh` installs the app.

## Licença

GPL-3.0-only, como o `cosmic-comp`.
