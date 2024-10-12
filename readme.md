- [Intro](#intro)
- [Usage](#usage)
  - [Ubuntu](#ubuntu)
  - [🦀 From sources](#-from-sources)
- [🗒️ ToDo](#️-todo)
- [Credits](#credits)
  - [Icons](#icons)
    - [The Noun Project http://thenounproject.com](#the-noun-project-httpthenounprojectcom)


# Intro
[pcb2gcode](https://github.com/pcb2gcode/pcb2gcode) is a great command-line tool, used to convert your gerber files into gcode for your milling machine. There's an (official) GUI for this tool called [pcb2gcodeGUI](https://github.com/pcb2gcode/pcb2gcodeGUI), which is also pretty awesome.

> But why create another GUI?

Well, there's no real reason, I wanted to learn how to use the Rust GUI library [iced](https://iced.rs/). The “real” improvement is the “Tool database” I've created to select the tool, instead of manually calculating the trace width. I'm also using the latest version of pcb2gcode without the deprecated options.

# Usage

## Ubuntu
You can download the last [release](https://github.com/jnthbdn/rs-pcb2gcode-gui/releases)

## 🦀 From sources
1. Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or see [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone or download this repository `git clone https://github.com/jnthbdn/rs-pcb2gcode-gui.git`
3. Enter to project folder `cd rs-pcb2gcode-gui`
4. Run from cargo `cargo run`

# 🗒️ ToDo
 - [ ] [GENRAL] Improve metric/imperial tools (How to alert user when he using imperial tool, when ouput file are in mm ?...)
 - [ ] [MILLING] Add `--draw-gerber-lines` support
 - [ ] [DRILLING] Support multidrill (`--drills-available`)


# Credits
## Icons
### The Noun Project [http://thenounproject.com](http://thenounproject.com)
 - [**Wan HD**](https://thenounproject.com/creator/wanhadian/) (https://thenounproject.com/creator/wanhadian/)
   + Arrow upward: [https://thenounproject.com/icon/arrow-upward-7052660/](https://thenounproject.com/icon/arrow-upward-7052660/)
   + Arrow downward: [https://thenounproject.com/icon/arrow-downward-7052668/](https://thenounproject.com/icon/arrow-downward-7052668/)
   + Add circle: [https://thenounproject.com/icon/add-circle-7052616/](https://thenounproject.com/icon/add-circle-7052616/)
   + Cancel: [https://thenounproject.com/icon/cancel-7052607/](https://thenounproject.com/icon/cancel-7052607/)
   + Autorenew: [https://thenounproject.com/icon/autorenew-7052654/](https://thenounproject.com/icon/autorenew-7052654/)

 - [Muhammad Ahsanu Nadia](https://thenounproject.com/creator/muhammad_ahsanu/)
   - Help: [https://thenounproject.com/icon/help-6778522/](https://thenounproject.com/icon/help-6778522/)