Name: millionaire
Version: 0.1.0
Release: 1
Summary: Web version of "Who Wants to Be a Millionaire?"
License: AGPL3.0
BuildArch: x86_64 
Requires: openssl

%description
Web version of "Who Wants to Be a Millionaire?"

%build
cp -R ${RPM_SOURCE_DIR}/src ${RPM_BUILD_DIR}
cp ${RPM_SOURCE_DIR}/Cargo.toml ${RPM_BUILD_DIR}
cargo build --release

%install
install -m 755 -d ${RPM_BUILD_ROOT}/usr/bin
cp target/release/chat ${RPM_BUILD_ROOT}/usr/bin/chat

%clean
rm -rf ${RPM_BUILD_ROOT}
rm -rf ${RPM_BUILD_DIR}

%files
/usr/bin/chat

