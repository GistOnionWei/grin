# Libercoin - Compilación, configuración y ejecución

## Plataformas soportadas

En un largo plazo, es probable que la mayoría de las plataformas sean compatibles en cierta medida.
El lenguaje de programación de Libercoin `rust` ha compilado metas para la mayoría de las plataformas.

¿Qué funciona hasta ahora?

* Linux x86\_64 y MacOS [libercoin + mining + development]
* Todavía no funciona con windows 10 [libercoin kind-of builds. No mining yet. Help wanted!]

## Requisitos

* rust 1.31+ (usa [rustup]((https://www.rustup.rs/))- por ejemplo, `curl https://sh.rustup.rs -sSf | sh; source $HOME/.cargo/env`)
  * Si rust está instalado, puede simplemente actualizar la versión con  `rustup update`
* clang
* ncurses y libs (ncurses, ncursesw5)
* zlib libs (zlib1g-dev or zlib-devel)
* pkg-config
* libssl-dev
* linux-headers (reportado como necesario en Alpine linux)
* llvm

Para las distribuciones basadas en Debian (Debian, Ubuntu, Mint, etc), todo en un comando (exceptuando Rust):

```sh
apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm
```

Para las Mac:

```sh
xcode-select --install
brew install --with-toolchain llvm
brew install pkg-config
brew install openssl
```

## Pasos para la compilación

```sh
git clone https://github.com/mimblewimble/libercoin.git
cd libercoin
cargo build --release
```

Libercoin también puede compilarse en modo debug (sin la etiqueta `--release`, pero usando la etiqueta `--debug` o `--verbose`) esto hará que la sincronización rápida sea excesivamente lenta debido a la gran sobrecarga de las operaciones criptográficas.

## Errores de compilación

Vea [Solución de problemas](https://github.com/mimblewimble/docs/wiki/Troubleshooting)

## ¿Qué se ha compilado?

Con una compilación finalizada se obtiene:

* `target/release/libercoin` - los binarios principales de libercoin

Todos los datos, configuración y archivos de registro creados y utilizados por Libercoin se encuentran en el directorio oculto `~/.libercoin` (bajo el directorio home del usuario) por defecto. Puede modificar toda la configuración editando el archivo `~/.libercoin/main/libercoin-server.toml`.

También es posible hacer que Libercoin cree sus propios archivos de datos en el directorio actual. Para ello ejecute:

```sh
libercoin server config
```

Lo que generará un archivo `libercoin-server.toml` en el directorio actual, preconfigurado para usar el directorio actual para todos sus datos. Ejecutando Libercoin desde un directorio que contiene el archivo `libercoin-server.toml` usará los valores de ese archivo en lugar de los valores por defecto de `~/.libercoin/main/libercoin-server.toml`.

Durante las pruebas, ponga el binario de Libercoin en su ruta de esta manera:

```sh
export PATH=/path/to/libercoin/dir/target/release:$PATH
```

Donde `path/to/libercoin/dir` es su ruta absoluta al directorio raíz de la instalación de Libercoin.

Puede ejecutar `libercoin` directamente (pruebe `libercoin help` para más opciones).

## Configuración

Libercoin se ejecuta con valores predeterminados, y puede configurarse aún más a través del archivo `libercoin-server.toml`. Este fichero es generado por libercoin en su primera ejecución, y contiene documentación sobre cada opción disponible.

Aunque se recomienda que realice toda la configuración de libercoin server a través de `libercoin-server.toml`, también es posible suministrar cambios de comandos para libercoin que anulan cualquier configuración en el archivo.

Para obtener ayuda sobre los comandos de libercoin y sus cambios intente:

```sh
libercoin help
libercoin wallet help
libercoin client help
```

## Docker

```sh
docker build -t libercoin -f etc/Dockerfile .
```

Puede ubicar la caché de Libercoin para que se ejecute dentro del contenedor

```sh
docker run -it -d -v $HOME/.libercoin:/root/.libercoin libercoin
```
## Compilación multiplataforma

Rust (cargo) puede compilar Libercoin para muchas plataformas, así que en teoría ejecutar `libercoin` como un nodo de validación en un dispositivo de baja potencia podría ser posible. Para hacer una compilación cruzada `libercoin` en una plataforma x86 Linux y generar binarios de ARM, por ejemplo para Raspberry-pi.

## Usando Libercoin

La página de la wiki [Cómo usar libercoin](https://github.com/mimblewimble/docs/wiki/How-to-use-libercoin) y las páginas de enlaces tienen más información sobre las características que disponemos, resolución de problemas, etc.

## Minando en Libercoin

Tenga en cuenta que todas las funciones de minería de Libercoin se han trasladado a un paquete independiente llamado [libercoin_minner](https://github.com/mimblewimble/libercoin-miner). Una vez que el nodo de libercoin esté listo y funcionando, puede empezar a minar compilando y ejecutando libercoin-miner con su nodo Libercoin en funcionamiento.
