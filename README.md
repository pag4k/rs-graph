# rs-graph
Rust implementation of Graph ADT using Adjacency List Structure

## Features
- Support only directed Graph.
- Vertices contain a variable of generic type.
- Vertices and egdes refered to using indices.
- Can be used in a wrapper struct to hide the use of these indices. See for example [rs-pathfinder library](https://github.com/pag4k/rs-pathfinder).

## Todo
- Add option to include a generic type to edges.
- Add option to remove vertices and edges.
- Replace indices by generational indices. This could be done using [generational-arena library](https://github.com/fitzgen/generational-arena).

## Authors

* **Pierre-André Gagnon** - *Initial work* - [pag4k](https://github.com/pag4k)

## License

- This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* Based on [Dr. Aiman Hanna](http://aimanhanna.com/concordia/) COMP352 Data Structure and Algorithm course notes.