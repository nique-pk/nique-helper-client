# Nique POS Helper client

Nique's own build of the open-source RustDesk client (AGPL-3.0, see LICENCE), pinned at
upstream 1.4.9 and rebranded for the Nique POS hub. The server address, its public key and
the app name are compiled in (libs/hbb_common/src/config.rs); the client reaches no other
server. English only. Windows x64 only. Built once by `.github/workflows/build-windows.yml`;
the built program is committed into the nique repository beside the hub installer.
