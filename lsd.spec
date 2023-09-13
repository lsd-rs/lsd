Name:           lsd
Version:        1.0.0
Release:        1%{?dist}
Summary:        The next gen ls command

License:        MIT
URL:            https://github.com/lsd-rs/lsd
Source0:        %{name}-%{version}.tar.gz

%description
This project is a rewrite of GNU ls with lots of added features like colors, icons, tree-view, more formatting options etc. The project is heavily inspired by the super colorls project.

%global debug_package %{nil} 

%prep
%setup -q

%build
cargo build --release

%install
%global _build_id_links none
mkdir -p %{buildroot}/%{_bindir}
# upx "target/release/lsd"
install -m 755 target/release/%{name} %{buildroot}/%{_bindir}/%{name}

%files
%defattr(-,root,root,-)
%{_bindir}/%{name}
