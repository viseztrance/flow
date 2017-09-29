%define _topdir     %(echo ~/rpmbuild)
%define name        flow
%define release     1
%define version     0.3.5
%define buildroot %{_topdir}/%{name}-%{version}-root
 
BuildRoot:      %{buildroot}
Name:           %{name}
Version:        %{version}
Release:        %{release}
Source:         %{version}.tar.gz
Summary:        %{name}
License:        GPLv3+

Group:		Applications/System
URL:		https://github.com/viseztrance/%{name}

BuildRequires: rust
BuildRequires: cargo
BuildRequires: ncurses-devel 
BuildRequires: readline-devel

%description
A tail like tool that can group and filter out data.

%prep
%setup -q

%build
cargo build --release --verbose

%install
mkdir -p $RPM_BUILD_ROOT/usr/bin
cp target/release/%{name} $RPM_BUILD_ROOT/usr/bin

%files
%{_bindir}/flow

%changelog

[daniel@localhost SPECS]$ cat flow.spec 
%define _topdir     %(echo ~/rpmbuild)
%define name        flow
%define release     1
%define version     0.3.5
%define buildroot %{_topdir}/%{name}-%{version}-root
 
BuildRoot:      %{buildroot}
Name:           %{name}
Version:        %{version}
Release:        %{release}
Source:         %{version}.tar.gz
Summary:        %{name}
License:        GPLv3+

Group:		Applications/System
URL:		https://github.com/viseztrance/%{name}

BuildRequires: rust
BuildRequires: cargo
BuildRequires: ncurses-devel 
BuildRequires: readline-devel

%description
A tail like tool that can group and filter out data.

%prep
%setup -q

%build
cargo build --release --verbose

%install
mkdir -p $RPM_BUILD_ROOT/usr/bin
cp target/release/%{name} $RPM_BUILD_ROOT/usr/bin

%files
%{_bindir}/flow

%changelog
