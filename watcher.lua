function is_rust(p)
   if p:find("target") ~= nil then return false end
   return p:ext() == "rs" or p:ext() == "toml"
end

PACKAGE="--all"

return {
   {
      should_run = is_rust,
      redirect_stderr = "/tmp/cargo.err",
      commands = {
         {
            name = "Running cargo check",
            command = "cargo check --all-targets " .. PACKAGE .. " --color=always",
         },
         {
            name = "Running cargo build [debug]",
            command = "cargo build " .. PACKAGE .. " --color=always",
         },
         -- {
            -- name = "Running cargo clippy",
            -- command = "cargo clippy " .. PACKAGE .. " --color=always",
         -- },
      }
   },
   {
      should_run = is_rust,
      redirect_stderr = "/dev/null",
      redirect_stdout = "/dev/null",
      commands = {
         {
            name = "Rusty tags",
            command = "rusty-tags vi",
         },

      }
   },
}
