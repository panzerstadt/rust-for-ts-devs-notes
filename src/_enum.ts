const enums = () => {
  enum Color {
    Red,
    Blue,
    Green,
  }

  const run = async (color: Color) => {
    switch (color) {
      case Color.Red:
        console.log("red");
        break;
      case Color.Blue:
        console.log("blue");
        break;
      case Color.Green:
        console.log("green");
        break;
    }
  };

  run(Color.Blue);
};
