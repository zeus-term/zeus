# Zeus High Level overview

Zeus application in breaken down into following 3 components

1. Terminal Proxy (**Hermes**) responsible for communicating commands and auto completion context between the terminal client and zeus master
2. Master (**Zeus**) which acts as a core auto completion engine itself, psuedo terminal manager and communicating what and where to render the recommendation box
3. Renderer (**Helios**) which is responsible for completion box configs like theming..., etc and rendering the recommendation box in the screen

## Architecture of zeus system
![Architecture of Zeus](./images/gilf-arch.drawio.svg)
