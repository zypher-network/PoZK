/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  // output: "export", //out

  async redirects() {
    return [
      {
        source: "/api/controllers",
        destination: "http://localhost:9098/api/controllers",
        permanent: false,
      },
    ];
  },
  reactStrictMode: false,
  webpack(config) {
    // Grab the existing rule that handles SVG imports
    const fileLoaderRule = config.module.rules.find((rule) =>
      rule.test?.test?.(".svg")
    );

    config.module.rules.push(
      // Reapply the existing rule, but only for svg imports ending in ?url
      {
        ...fileLoaderRule,
        test: /\.svg$/i,
        resourceQuery: /url/, // *.svg?url
      },
      // Convert all other *.svg imports to React components
      {
        test: /\.svg$/i,
        issuer: fileLoaderRule.issuer,
        resourceQuery: { not: [...fileLoaderRule.resourceQuery.not, /url/] }, // exclude if *.svg?url
        // use: ["@svgr/webpack"],
        use: [{
          loader: '@svgr/webpack',
          options: {
            svgoConfig: {
              plugins: [
                {
                  name: 'preset-default',
                  params: {
                    overrides: {
                      removeViewBox: false,
                    },
                  },
                },
              ],
            },
          }
        }],
      }
    );

    // Modify the file loader rule to ignore *.svg, since we have it handled now.
    fileLoaderRule.exclude = /\.svg$/i;

    config.resolve.fallback = { fs: false, net: false, tls: false };
    return config;
  },
  env: {
    API_BASE_URL: process.env.API_BASE_URL,
  }
};

export default nextConfig;
