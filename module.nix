{self}: {
  config,
  lib,
  modulesPath,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    types
    ;

  cfg = config.services.rime;

  settingsFormat = pkgs.formats.yaml {};
  configFile = settingsFormat.generate "rime-config.yaml" cfg.settings;

  forgesSubmodule = {
    freeformType = settingsFormat.type;

    options = {
      api_page_size = mkOption {
        type = types.int;
        default = 10;
      };
    };
  };

  settingsSubmodule = {
    freeformType = settingsFormat.type;

    options = {
      addr = mkOption {
        type = types.str;
        default = "0.0.0.0";
      };

      port = mkOption {
        type = types.str;
        default = "3000";
      };

      forges = mkOption {
        type = types.submodule forgesSubmodule;
        default = {};
      };
    };
  };
in {
  options.services.rime = {
    enable = mkEnableOption "rime";

    package = mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "rime" {default = "default";};

    settings = mkOption {
      type = types.submodule settingsSubmodule;
      default = {};
      description = ''
        Settings for rime.

        See the source code for more:
        https://github.com/cafkafk/rime/blob/b519daf1a4d4eacfbb89251d6ed4dc223a0bebb1/src/data/config.rs#L20-L36
      '';
    };

    domain = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = ''
        If non-null, enables an NGINX reverse proxy virtual host at this FQDN.
      '';
      example = "rime.example.com";
    };

    nginx = mkOption {
      type = types.submodule (
        import (modulesPath + "/services/web-servers/nginx/vhost-options.nix") {inherit config lib;}
      );
      default = {};
      example = {
        enableACME = true;
        forceHttps = true;
      };
      description = ''
        Configuration for the NGINX virtual host.
      '';
    };
  };

  config = mkIf cfg.enable {
    services.nginx.virtualHosts.${cfg.domain} = lib.mkIf (cfg.domain != null) (
      lib.mkMerge [
        {
          locations."/" = {
            proxyPass = "http://${cfg.settings.addr}:${cfg.settings.port}";
          };
        }

        cfg.nginx
      ]
    );

    systemd.services.rime = {
      description = "Nix Flake Input Versioning";

      after = ["network.target"];
      wants = ["network.target"];

      path = [cfg.package];

      script = ''
        ${lib.getExe cfg.package} --config ${configFile}
      '';

      serviceConfig = {
        Type = "simple";
        Restart = "on-failure";

        DynamicUser = true;

        # Hardening
        CapabilityBoundingSet = [""];
        DeviceAllow = [""];
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateIPC = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProcSubset = "pid";
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProtectSystem = "strict";
        RestrictAddressFamilies = [
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = ["native"];
        SystemCallFilter = [
          "@system-service"

          "~@privileged"
          "~@resources"
        ];
        UMask = "0077";
      };
    };
  };
}
