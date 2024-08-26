import { Metadata } from "next";

interface IMetadata {
  description?: string;
  path?: string;
  tit?: string;
  image?: string;
}
const websiteURL = "https://pozk.zypher.network";
const siteName = "zypher.network";
const locale = "en_GB";
export default function generateMetadata(props?: IMetadata): Metadata {
  const { tit, description: desc, path, image } = props ?? {};
  const description = desc ? desc + " | PoZK" : "PoZK";

  const images = image ? `${websiteURL}${image}` : "/summary_large_image.jpg";
  const title = `${tit ? tit + " | PoZK" : "PoZK"}`;
  return {
    title,
    description,
    icons: "/favicon.svg",
    metadataBase: process.env.metadataBase
      ? new URL(process.env.metadataBase)
      : undefined,
    openGraph: {
      title,
      description,
      url: websiteURL + (path ?? ""),
      siteName,
      images,
      locale,
    },
    twitter: {
      card: "summary_large_image",
      title,
      description,
      images,
    },
  };
}
