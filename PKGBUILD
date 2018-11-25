# Maintainer: Peltoche <dev@halium.fr>
pkgname=lsd
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A ls command with a lot of pretty colors."
url="https://github.com/Peltoche/lsd"
license=('Apache-2.0')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/Peltoche/lsd
}
