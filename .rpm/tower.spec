%define _spec_dir %(echo $PWD/.rpm)/
%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Prefix: /opt
%define _prefix /opt/

Name: tower
Summary: Tower is a simple viewer made for cg animation and vfx, with some new concepts.
Version: @@VERSION@@
Release: @@RELEASE@@
License: ASL 2.0 or MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/opt/tower/config
cp %{_specdir}/../../../../config/display.ron %{buildroot}/opt/tower/config
cp -a * %{buildroot}/opt/tower/

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
%defattr(755,root,root,-)
%dir /opt/tower/config
%defattr(-,root,root,-)
/opt/tower/config/display.ron
