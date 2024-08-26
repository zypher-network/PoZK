import BigNumberJs, { BigNumber } from "bignumber.js";
export const BM18 = new BigNumberJs("10").exponentiatedBy(18);
export const FORMAT = {
  decimalSeparator: ".",
  groupSeparator: ",",
  groupSize: 3,
  secondaryGroupSize: 0,
  fractionGroupSeparator: " ",
  fractionGroupSize: 0,
  suffix: "M",
  prefixes: {
    "-": "",
    "+": "",
  },
  abbreviations: {
    K: "K",
    M: "M",
    B: "B",
    T: "T",
  },
};
BigNumber.config({
  DECIMAL_PLACES: 2,
  ROUNDING_MODE: BigNumber.ROUND_HALF_DOWN,
});
export default BigNumberJs;
