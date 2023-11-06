FROM gitpod/workspace-full:latest

RUN rm \
        /etc/apache2/mods-enabled/mpm_prefork.conf \
        /etc/apache2/mods-enabled/mpm_prefork.load

RUN yes Y | sudo install-packages apache2

RUN sudo install-packages \
        pkg-config \
        build-essential \
        curl \
        wget \
        file \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libdbus-1-dev \
        libsoup2.4 \
        libwebkit2gtk-4.0-dev \
        patchelf
