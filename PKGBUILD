# Maintainer: Thomas Judkins <tjud14@gmail.com>
pkgname=yoink
pkgver=0.1.0
pkgrel=1
pkgdesc="Command-line utility for recursively copying directory structure & file contents to your clipboard"
arch=('x86_64' 'aarch64')
url="https://github.com/yourusername/yoink"
license=('GPL3')
depends=('gcc-libs' 'xclip')
optdepends=('wl-clipboard: For Wayland support'
            'xsel: Alternative X11 clipboard manager'
            'xdotool: Fallback clipboard method'
            'termux-api: For Android/Termux support')
makedepends=('cargo' 'git')
source=("git+${url}.git")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release
}

check() {
  cd "$pkgname"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen
}

package() {
  cd "$pkgname"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
} 