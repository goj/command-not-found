{
  inputs = {
    dream2nix.url = "github:nix-community/dream2nix";
  };

  outputs = {
    self,
    dream2nix,
  } : (dream2nix.lib.makeFlakeOutputs {
      systems = ["x86_64-linux"];
      config = {
        disableIfdWarning = true;
        projectRoot = ./.;
      };
      source = ./.;
      settings = [
        {
          builder = "crane";
          translator = "cargo-lock";
        }
      ];
    });
}
