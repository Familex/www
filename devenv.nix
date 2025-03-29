{pkgs, ...}: {
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = with pkgs; [
    systemfd
    cargo-watch
  ];

  scripts = {
    dev-up.exec = ''
      systemfd --no-pid -s http::3000 -- cargo watch -x run
    '';
  };
}
