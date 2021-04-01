import math

try:
    from proc_blocks import Fft
except Exception as e:
    print(
        "ERROR: Unable to import the native module. You may need to install it or run `maturin develop`"
    )
    raise e


def use_fft():
    # Create our FFT analyser with a sample rate of 360Hz
    fft = Fft(360)
    # Make a sine wave with a 360-sample period (i.e. 1Hz) and an amplitude of
    # 100, rounded to the nearest integer
    samples = [round(math.sin(i) * 100) for i in range(0, 128)]
    # Calculate the spectrum
    spectrum = fft(samples)
    # And print the results
    print(spectrum[:10])


def main():
    use_fft()


if __name__ == "__main__":
    main()
