const tree_references = {
  children: [
    {
      name: "Reference documentation",
      url: "/references",
    },
    {
      name: "API Reference",
      url: "/references/api",
      children: [
        {
          name: "Daemon",
          url: "/references/api/daemon",
          children: [
            {
              name: "Latest",
              url: "/references/api/daemon/latest"
            }
          ]
        },
      ]
    },
    {
      name: "CLI Reference",
      url: "/references/cli",
      children: [
        {
          name: "Nanocl CLI (nanocl)",
          url: "/references/cli/nanocl",
          children: [
            {
              name: "Base",
              url: "/references/cli/nanocl/base"
            }
          ]
        },
      ]
    }
  ]
}

module.exports = {
  header_links: [
    {
      title: "man",
      url: "/man",
    },
  ],
  routes: {
    "/man": {
      title: "Man",
      tree: tree_references,
    }
  },
  home_page_blocks: [
  ]
}
