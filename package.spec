Name: jnferner
Version: 0.5.6
Release: 1
Summary: homepage of Jan Nils Ferner
License: AGPL3.0
BuildArch: x86_64 
Requires: openssl

%description
homepage of Jan Nils Ferner

%build
cp -R ${RPM_SOURCE_DIR}/src ${RPM_BUILD_DIR}
cp -R ${RPM_SOURCE_DIR}/res ${RPM_BUILD_DIR}
cp ${RPM_SOURCE_DIR}/Cargo.toml ${RPM_BUILD_DIR}
cargo build --release

%install
install -m 755 -d ${RPM_BUILD_ROOT}/opt/homepage
install -m 755 -d ${RPM_BUILD_ROOT}/etc/systemd/system
install -m 755 -d ${RPM_BUILD_ROOT}/var/www/html
cp target/release/homepage ${RPM_BUILD_ROOT}/opt/homepage/
cp -R res/public ${RPM_BUILD_ROOT}/var/www/html/
cp -R res ${RPM_BUILD_ROOT}/opt/homepage/

%post
systemctl daemon-reload
systemctl restart homepage

%clean
rm -rf ${RPM_BUILD_ROOT}
rm -rf ${RPM_BUILD_DIR}

%files
/opt/homepage/
/var/www/html/
