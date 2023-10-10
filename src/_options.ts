const ensureNumber2 = (input: number | undefined): number => {
  if (typeof input === "number") {
    return input * 5;
  }
  return 0;
};
