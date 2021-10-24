# Parse Azure Indexer Definition

**note:** this is still very much a WIP.

When moving some services to TS we were lacking a correct definition of some values.

Since the single point of truth was the Azure Indexer definition, this small CLI is a small tool to quickly convert from said Indexer to a TS Interface to aid in the migration.

TODO:

- [x]: parse a json config file into Typescript Interface.
- []: fetch indexer from azure.
- [x]: a CLI to set up the config for fetching.
- []: documentation.
