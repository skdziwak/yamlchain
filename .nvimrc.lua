local pwd = vim.fn.getcwd()
return {
  yaml_schemas = {
    ["file://" .. pwd .. "/workflows-schema.json"] = "yc-*.yaml"
  }
}
