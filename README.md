# chrome_offline_game

chrome offline game in rust to create a custom genetic algorithm

## installation

### GTK

Don't forget to add dependancies to build ui (gtk-rs) : `sudo apt install libgtk-4-dev build-essential`

get the version installed : `pkg-config --modversion gtk4` and run `cargo add gtk4 --rename gtk --features v{version}` to get the crate (more info [here](https://gtk-rs.org/gtk4-rs/stable/latest/book/project_setup.html))

Note : this library sucked, use Iced instead

### Iced

more information [here](https://github.com/iced-rs/iced).

#### WSL install

Don't forget to install [openGL](https://gist.github.com/Mluckydwyer/8df7782b1a6a040e5d01305222149f3c) : run the command :

```shell
sudo apt install mesa-utils libglu1-mesa-dev freeglut3-dev mesa-common-dev
```
