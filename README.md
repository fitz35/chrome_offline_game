# chrome_offline_game

chrome offline game in rust to create a custom genetic algorithm

## installation

### GTK

Don't forget to add dependancies to build ui (gtk-rs) : `sudo apt install libgtk-4-dev build-essential`

get the version installed : `pkg-config --modversion gtk4` and run `cargo add gtk4 --rename gtk --features v{version}` to get the crate (more info [here](https://gtk-rs.org/gtk4-rs/stable/latest/book/project_setup.html))

Note : this library sucked, use Iced instead

### Iced

more information [here](https://github.com/iced-rs/iced). The module "tokio" is for the subscription. To have access todebug, press f12

#### WSL install

Don't forget to install [openGL](https://gist.github.com/Mluckydwyer/8df7782b1a6a040e5d01305222149f3c) : run the command :

```shell
sudo apt install mesa-utils libglu1-mesa-dev freeglut3-dev mesa-common-dev
```

## Random

Use the [rand_pcg](https://crates.io/crates/rand_pcg) crate to save it with serialise/deserialise.

WARN : Don't use hashSet to

## Program params

There is 3 used of this program :

* To play : add `-p` this will let the user play
* To see brain play : add `-b path/to/the/brain` this will display the brain and let play
* To train the brains : add `-t path/to/the/training/folder` this will train (from new or continue from old)

to precise the option, add `-o path/to/option/json` . (only work for play, and brain play, the training option are in folder )
