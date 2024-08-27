import { Kanit } from "next/font/google";

const kanit = Kanit({
  subsets: ["latin"],
  variable: "--font-kanit",
  weight: ["300", "400", "500", "600", "700", "800", "900"],
  display: 'swap',
});
export { kanit };
