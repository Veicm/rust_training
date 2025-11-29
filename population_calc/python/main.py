from tqdm import tqdm


class Simulator:
    """
    This class is used to simulate the german population over the years
    since 2005.

    Attributes:
        None
    """

    def __init__(self) -> None:
        self.start_year: int = 2005
        self.values: tuple[float, float, float, float] = (12.3, 39.1, 15.5, 16.3)

    def sim(self, years: int, raw: bool = False) -> None:
        """
        This function prints the values for each given year.

        Args:
            years (int): The sum of the years you want to simulate.
            raw (bool): If true the function prints values for every year.
                defaults to False.

        Returns:
            None
        """
        if raw:
            print("Year: 2005")

            print(f"0-14 years:     {self.values[0]:.8f} million")
            print(f"15-49 years:    {self.values[1]:.8f} million")
            print(f"50-64 years:    {self.values[2]:.8f} million")
            print(f"over 65 years:  {self.values[3]:.8f} million")
            print("")
        for year in tqdm(range(years), desc="Processing..."):
            if raw:
                print(f"Year: {self.start_year + year + 1}")

            self.values = self.calc(self.values)
            if raw:
                print(f"0-14 years:     {self.values[0]:.8f} million")
                print(f"15-49 years:    {self.values[1]:.8f} million")
                print(f"50-64 years:    {self.values[2]:.8f} million")
                print(f"over 65 years:  {self.values[3]:.8f} million")
                print("")
        if not raw:
            print(f"Year: {self.start_year + years}:")
            print(f"0-14 years:     {self.values[0]:.8f} million")
            print(f"15-49 years:    {self.values[1]:.8f} million")
            print(f"50-64 years:    {self.values[2]:.8f} million")
            print(f"over 65 years:  {self.values[3]:.8f} million")
            print("")

    def calc(
        self, old_values: tuple[float, float, float, float]
    ) -> tuple[float, float, float, float]:
        """
        This function calculates the new values based on the old values, using
        a static schema.

        Args:
            old_values (tuple[float, float, float, float]): The values
                from the past year.

        Returns:
            tuple[float, float, float, float]: The new values for the present year.
        """
        g_0_14, g_15_49, g_50_64, g_65 = old_values

        # Group 0-14
        new_0_14: float = g_0_14 * 0.93
        new_0_14 += g_15_49 * 0.02

        # Group 15-49
        new_15_49: float = g_15_49 * 0.97
        new_15_49 += g_0_14 * 0.066

        # Group 50-64
        new_50_64: float = g_50_64 * 0.925
        new_50_64 += g_15_49 * 0.029

        # Group over 65
        new_65: float = g_65 * 0.972
        new_65 += g_50_64 * 0.066

        new_values: tuple[float, float, float, float] = (
            new_0_14,
            new_15_49,
            new_50_64,
            new_65,
        )
        return new_values

    def start_point(self) -> None:
        print("How many years do you want to simulate?")
        print("integers only !!!")
        years: int = int(input())
        question: str = str(input("Do you want an output for each year? [y/N] "))
        if question == "y" or question == "Y":
            raw: bool = True
        else:
            raw: bool = False
        self.sim(years, raw)


def main() -> None:
    simulator = Simulator()
    simulator.start_point()


if __name__ == "__main__":
    main()
