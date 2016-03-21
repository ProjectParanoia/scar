# `before_deploy` phase: here we package the build artifacts

set -ex

# Generate artifacts for release
mk_artifacts() {
  cargo build --target $TARGET --release
}

mk_tarball() {
  # create a "staging" directory
  local temp_dir=$(mktemp -d)
  local out_dir=$(pwd)

  # NOTE All Cargo build artifacts will be under the 'target/$TARGET/{debug,release}'
  cp target/$TARGET/release/scar $temp_dir

  pushd $temp_dir

  echo $out_dir


  # release tarball will look like 'rust-everywhere-v1.2.3-x86_64-unknown-linux-gnu.tar.gz'
  tar czf ~/${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz *

  echo "Tarball generated in $out_dir/${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz"

  popd $temp_dir
  rm -r $temp_dir
}

main() {
  mk_artifacts
  mk_tarball
}
