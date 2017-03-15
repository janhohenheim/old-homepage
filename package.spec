Name: millionaire
Version: 0.4.0
Release: 1
Summary: Web version of "Who Wants to Be a Millionaire?"
License: AGPL3.0
BuildArch: x86_64 
Requires: openssl

%description
Web version of "Who Wants to Be a Millionaire?"

%build
cp -R ${RPM_SOURCE_DIR}/src ${RPM_BUILD_DIR}
cp -R ${RPM_SOURCE_DIR}/res ${RPM_BUILD_DIR}
cp ${RPM_SOURCE_DIR}/Cargo.toml ${RPM_BUILD_DIR}
cp -R ${RPM_SOURCE_DIR}/contrib ${RPM_BUILD_DIR}
cargo build --release

%install
install -m 755 -d ${RPM_BUILD_ROOT}/opt/chat
install -m 755 -d ${RPM_BUILD_ROOT}/etc/systemd/system
cp target/release/chat ${RPM_BUILD_ROOT}/opt/chat/
cp -R res ${RPM_BUILD_ROOT}/opt/chat/
cp contrib/chat.service ${RPM_BUILD_ROOT}/etc/systemd/system/

%post
systemctl daemon-reload
systemctl restart chat

%clean
rm -rf ${RPM_BUILD_ROOT}
rm -rf ${RPM_BUILD_DIR}

%files
/opt/chat/
/etc/systemd/system/chat.service

