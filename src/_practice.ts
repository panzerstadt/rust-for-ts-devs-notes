const practice = (values: number[], index: number): number => {
  if (values[index]) return values[index]! * 5;
  return index * 5;
};

const vector = [3, 2, 1];
console.log(practice(vector, 0));
console.log(practice(vector, 5));
