const { default: original } = require("@changesets/changelog-github");

module.exports = {
  getDependencyReleaseLine: (changesets, dependenciesUpdated, options) => {
    // We don't want to embed NAPI dependency updates (always synced):
    dependenciesUpdated = [];

    return original.getDependencyReleaseLine(changesets, dependenciesUpdated, options);
  },

  getReleaseLine: (changeset, type, options) => {
    return original.getReleaseLine(changeset, type, options);
  },
};
