# mark version
cargo install cargo-bump
cargo bump "${VERSION}"

# make a crate
mkdir -p "${GITHUB_WORKSPACE}/temp"
cargo package --allow-dirty --target-dir="${GITHUB_WORKSPACE}/temp"

# save the output filename to env
PACKAGE_FILENAME=$(cd "${GITHUB_WORKSPACE}/temp/package" && ls -1 -- *.crate)
echo "PACKAGE_FILENAME=${PACKAGE_FILENAME}" >> "${GITHUB_ENV}"

# move package to dist
mkdir -p "${GITHUB_WORKSPACE}/dist"
mv "${GITHUB_WORKSPACE}/temp/package/${PACKAGE_FILENAME}" "${GITHUB_WORKSPACE}/dist/${PACKAGE_FILENAME}"
