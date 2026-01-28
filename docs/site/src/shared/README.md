# Haneul Labs Shared Docusaurus Documentation Components

This repo houses the shared custom components, plugins, and scripts used across all Haneul Stack documentation sites (Haneul, Walrus, Seal, HaneulNS, etc).

This repo is a work in progress and will continue to be updated, as some sites have not yet adopted the Docusaurus framework.

## Shared components

The shared components for all sites are:
```
├── components/
│   ├── Cards/
│   ├── ExampleImport/
│   ├── Glossary/
│   ├── ImportContent/
│   ├── RelatedLink/
│   ├── SidebarIframe/
│   ├── Snippet/
│   ├── ThemeToggle/
│   └── UnsafeLink/
├── css/
│   └── details.css
├── js/
│   ├── convert-release-notes.js
│   ├── tabs-md.client.js
│   └── utils.js
├── plugins/
│   ├── descriptions/
│   ├── inject-code/
│   ├── plausible/
│   ├── tabs-md-client/
│   │   └── index.mjs
│   └── remark-glossary.js
```

## Components that cannot be shared

Despite the sites using the same plugins and components for:

1. Cookbook AI (`plugins/askcookbook`)
2. Algolia Search (`components/Search`)
3. Push Feedback

Each of these has a custom configuration for their own API keys. These components are
thus managed individually.

Additionally, all `src/theme` and `css/` components are unique to each site to prevent conflicts
between the styling of each individual site.

## Haneul-specific components

Components unique to the Haneul documentation are as follows:

```
├── client
│   ├── pushfeedback-toc.js
├── components/
│   └── API
│   └── BetaTag
│   └── EffortBox
│   └── GetStartedLink
│   └── GraphqlBetaLink
│   └── HomepageFeatures
│   └── Protocol
│   └── ProtocolConfig
├── css/
│   └── custom.css
│   └── fonts.css
├── js/
│   └── convert-awesome-haneul.mjs
│   └── update-cli-output.js
├── plugins/
│   └── askcookbook
│   └── betatag
│   └── effort
│   └── framework
│   └── protocol
```

## Walrus-specific components

Components unique to the Walrus documentation are as follows:

```
docs/site/src/
├── components/
│   ├── HomepageFeatures/
│   ├── OperatorsList/
│   ├── PortalsList/
│   ├── PushFeedback/
│   └── Search/
├── css/
│   ├── cards.module.css
│   ├── custom.css
│   ├── fontawesome.ts
│   ├── fonts.css
│   └── sidebar.module.css
├── pages/
├── plugins/
│   ├── askcookbook/
│   ├── client/
│   ├── index.ts
│   └── tailwind-config.js
└── scripts/
    ├── copy-yaml-files.js
    └── generate-import-context.js
```
