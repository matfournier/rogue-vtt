/* eslint-disable import/no-extraneous-dependencies */
// silence `'vite' should be listed in project's dependencies, not devDependencies`

import glob from "glob";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import { viteStaticCopy } from "vite-plugin-static-copy";
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from "path"
// import from "path" and "fs" causes eslint to crash for some reason
// const path = require("path");

/**
 *
 * @returns {import("vite").PluginOption}
 */
function prodScriptPlugin() {
  return {
    name: "prod-script",
    // TODO: think theres some weird caching shit or race condition going on:
    // Had to rm node_modules and reinstall deps or else assets/404-[hash].css
    // is generated instead of assets/index-[hash].css -> base.html not having
    // a <link rel="stylesheet" href="/assets/index-[hash].css" />
    transform(code, id) {
      if (id.endsWith(".html")) {
        return (
          code
            // remove vite client import
            .replace(
              /<script type="module">\s*import "http:\/\/localhost:5173\/@vite\/client"[\s\S]*?<\/script>/,
              "",
            )
            // remove vite dev server url from everywhere (paths for js, css, assets)
            .replaceAll("http://localhost:5173", "")
        );
      }
      return code;
    },
  };
}

/**
 *
 * @returns {{ [entryAlias: string]: string }} //
 */
function htmlInputs() {
  const htmlFiles = glob
    .sync(path.join(__dirname, "/**/*.html"))
    .filter(
      (htmlFilePath) =>
        !htmlFilePath.includes("dist/") &&
        !htmlFilePath.includes("node_modules/"),
    );
  return Object.fromEntries(
    htmlFiles.map((htmlFilePath) => {
      const pathSegments = htmlFilePath.split(path.sep);
      while (pathSegments[0] !== "app") {
        pathSegments.shift();
      }
      pathSegments.shift(); // remove leading app/

      const baseName = pathSegments[pathSegments.length - 1];
      if (!baseName) {
        throw new Error(`Error parsing path: ${htmlFilePath}`);
      }
      const baseNameNoExt = baseName.replace(path.extname(baseName), "");
      pathSegments[pathSegments.length - 1] = baseNameNoExt;
      const alias = pathSegments.join(path.sep);

      return [alias, htmlFilePath];
    }),
  );
}

export default defineConfig(() => ({
  /** @type {"mpa"} */
  appType: "mpa",
  build: {
    // include source maps if env var set to true
    sourcemap: process.env.SOURCE_MAP === "true",
    rollupOptions: {
      input: htmlInputs(),
    },
  },
  // we want to preserve the same directory structure between / at dev time and
  // dist/ at prod time, so copy static files manually with viteStaticCopy
  // instead of using the public/ dir
  /** @type {false} */
  publicDir: false,
  plugins: [
    svelte(),
    viteStaticCopy({
      targets: [
        { src: "robots.txt", dest: "" },
        { src: "favicon.ico", dest: "" },
        { src: "images/", dest: "" },
      ],
    }),
    VitePWA({
      workbox: {
        // no point precaching html templates on client
        globIgnores: ["templates/**"],
        globPatterns: [
          "**/*.{js,css,html}", // default
          "assets/**",
        ],
      },
      includeAssets: [`favicon.ico`, `images/logo/apple-touch-icon.png`],
      manifest: {
        name: "My Web App",
        short_name: "mwa",
        description: "This is a test web app",
        //
        icons: [
          {
            src: `images/logo/logo_512x512.png`,
            sizes: "512x512",
            type: "image/png",
          },
          {
            src: `images/logo/logo_192x192.png`,
            sizes: "192x192",
            type: "image/png",
          },
          {
            src: `images/logo/logo_192x192.png`,
            sizes: "192x192",
            type: "image/png",
            purpose: "any maskable",
          },
        ],
        display: "fullscreen",
        // TODO: USE ACTUAL THEME COLORS
        theme_color: "#FFFFFF",
        background_color: "#FFFFFF",
      },
    }),
    prodScriptPlugin(),
  ],
}));