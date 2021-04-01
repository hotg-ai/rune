import math
import random

try:
    from proc_blocks import Fft, Normalize
except Exception as e:
    print(
        "ERROR: Unable to import the native module. You may need to install it or run `maturin develop`"
    )
    raise e


def use_normalize():
    norm = Normalize()
    # Create some random data
    data = [round(random.uniform(-100, 100)) for i in range(0, 5)]
    print("Original:", data)
    normalized = norm(data)
    print("Normalized:", normalized)


def use_fft():
    # Create our FFT analyser with a sample rate of 360Hz
    fft = Fft(360)
    # Make a sine wave with a 180-sample period (i.e. 2Hz) and an amplitude of
    # 100, rounded to the nearest integer
    samples = [round(math.sin(i * 2) * 100) for i in range(0, 1960)]
    # Calculate the spectrum
    spectrum = fft(samples)
    # And print the results
    n = 10
    print(f"First {n} frequencies: {spectrum[:n]}")


def main():
    use_normalize()
    use_fft()


if __name__ == "__main__":
    main()
