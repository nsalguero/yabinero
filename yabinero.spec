%global debug_package %{nil}

%define name yabinero
%define version 1.9.0
%define release %mkrel 1

Summary: Yet Another Binero Puzzle

Name: %{name}
Version: %{version}
Release: %{release}
Source0: https://github.com/nsalguero/%{name}/archive/v%{version}.tar.gz
# ex: tar xzf v1.0.0.tar.gz; cd yabinero-1.0.0; cargo vendor; tar cvJf ../yabinero-cargo-vendor-1.0.0.tar.xz vendor/
Source1: %{name}-cargo-vendor-%{version}.tar.xz
Source2: cargo.config

License: GPLv3+
Group: Games/Puzzles
Url: https://github.com/nsalguero/%{name}

BuildRequires: desktop-file-utils
BuildRequires: cargo
BuildRequires: cmake
BuildRequires: gettext
BuildRequires: pkgconfig(alsa)
BuildRequires: pkgconfig(x11)
BuildRequires: pkgconfig(glu)
BuildRequires: pkgconfig(xinerama)
BuildRequires: pkgconfig(xcursor)
BuildRequires: pkgconfig(xrender)
BuildRequires: pkgconfig(xft)
BuildRequires: pkgconfig(fontconfig)

%description
This software can generate and solve binero puzzles.

The icon was created by the author of this game using GIMP.  The other icons
come from Openclipart.

The two ogg files come from the conversion of WAVE files that come from K3B.

%prep
%autosetup

%__mkdir_p .cargo
cp %{S:2} .cargo/config
tar xf %{S:1}

%build
cargo build --release

cat > %{name}-wrapper << "EOF"
#!/bin/sh
cd %{_datadir}/games/%{name}
exec ./%{name}
EOF

cat > %{name}.desktop << "EOF"
[Desktop Entry]
Encoding=UTF-8
Name=Binero Game
Name[fr]=Jeu de Binero
Comment=Yet Another Binero Game
Comment[fr]=Encore Un Jeu de Binero
Exec=%{_prefix}/games/%{name}
Icon=%{name}
Terminal=false
Type=Application
Categories=Game;LogicGame;
EOF

%install
mkdir -p %{buildroot}/%{_datadir}/games/%{name}
install -m 0755 target/release/%{name} %{buildroot}/%{_datadir}/games/%{name}

install -m 0644 LICENSE %{buildroot}/%{_datadir}/games/%{name}

mkdir -p %{buildroot}/%{_datadir}/games/%{name}/icons
install -m 0644 icons/* %{buildroot}/%{_datadir}/games/%{name}/icons

mkdir -p %{buildroot}/%{_datadir}/games/%{name}/locale/fr/LC_MESSAGES
install -m 0644 locale/fr/LC_MESSAGES/%{name}.mo %{buildroot}/%{_datadir}/games/%{name}/locale/fr/LC_MESSAGES

mkdir -p %{buildroot}/%{_datadir}/games/%{name}/sounds
install -m 0644 sounds/*.ogg %{buildroot}/%{_datadir}/games/%{name}/sounds

mkdir -p %{buildroot}/%{_iconsdir}
install -m 0644 icons/icon.png %{buildroot}/%{_iconsdir}/%{name}.png

mkdir -p %{buildroot}/%{_prefix}/games
install -m 0755 %{name}-wrapper %{buildroot}/%{_prefix}/games/%{name}

desktop-file-install --vendor="" \
                     --dir=%{buildroot}/%{_datadir}/applications/ \
                     %{name}.desktop

mkdir -p %{buildroot}/%{_menudir}
cat > %{buildroot}/%{_menudir}/%{name} << "EOF"
?package(%name):\
command="%{_prefix}/games/%{name}"\
icon="%{name}"\
title="Binero Game"\
longtitle="Yet Another Binero Game"\
needs="x11"\
section="Game" \
xdg="true"
EOF

%files
%doc README.md
%license LICENSE

%{_datadir}/games/%{name}/%{name}
%{_datadir}/games/%{name}/LICENSE
%{_datadir}/games/%{name}/icons/icon.png
%{_datadir}/games/%{name}/icons/*.svg
%{_datadir}/games/%{name}/locale/fr/LC_MESSAGES/%{name}.mo
%{_datadir}/games/%{name}/sounds/*.ogg

%{_iconsdir}/%{name}.png

%{_prefix}/games/%{name}

%{_datadir}/applications/%{name}.desktop
%{_menudir}/%{name}

%changelog
* Thu Jul 16 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.9.0-1.mga7
- improve GUI

* Mon Jul 13 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.8.0-1.mga7
- improve GUI

* Wed Jul 08 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.7.0-1.mga7
- improve GUI

* Tue Jul 07 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.6.0-1.mga7
- improve GUI code

* Mon Jul 06 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.5.0-1.mga7
- improve GUI

* Tue Jun 23 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.4.0-1.mga7
- simplify GUI code

* Mon Jun 22 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.3.0-1.mga7
- fix a bug in the engine

* Fri Jun 19 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.2.0-1.mga7
- fix some bugs in the GUI

* Thu Jun 18 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.1.0-1.mga7
- improve GUI and allow the user to choose some colors

* Fri Jun 12 2020 Nicolas Salguero <nicolas.salguero@laposte.net> 1.0.0-1.mga7
- initial build
