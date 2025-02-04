import tags from "~/generated/state.json";

export const useTags = () =>
  useState("tags", () => ({
    categories: tags.categories,
    loaders: tags.loaders,
    gameVersions: tags.gameVersions,
    donationPlatforms: tags.donationPlatforms,
    reportTypes: tags.reportTypes,
    projectTypes: [
      {
        actual: "mod",
        id: "mod",
        display: "mod",
      },
      {
        actual: "mod",
        id: "plugin",
        display: "plugin",
      },
    ],
    loaderData: {
      pluginLoaders: ["bukkit", "spigot", "paper", "purpur", "sponge", "folia",/*new*/ "obsidian"],
      pluginPlatformLoaders: ["bungeecord", "waterfall", "velocity"],
      allPluginLoaders: [
        "bukkit",
        "spigot",
        "paper",
        "purpur",
        "sponge",
        "bungeecord",
        "waterfall",
        "velocity",
        "folia",
        // new
        "obsidian",
      ],
      dataPackLoaders: [],
      modLoaders: [],
      hiddenModLoaders: [],
    },
    projectViewModes: ["list", "grid", "gallery"],
    approvedStatuses: ["approved", "archived", "unlisted", "private"],
    rejectedStatuses: ["rejected", "withheld"],
    staffRoles: ["moderator", "admin"],
  }));
