/**
 * Shorten wallet address
 * @param address wallet address
 * @param size if size is 'normal', the address will be shorten to 8...8, if size is 'shorter', the address will be shorten to 5...3
 * @returns
 */
export function shortenWalletAddress(
  address: string,
  size: "normal" | "shorter"
): string {
  if (size === "shorter") {
    if (address.length <= 8) {
      return address;
    }
    return `${address.slice(0, 5)}...${address.slice(-4)}`;
  } else if (size === "normal") {
    if (address.length <= 16) {
      return address;
    }
    return `${address.slice(0, 8)}...${address.slice(-8)}`;
  }
  return address;
}
