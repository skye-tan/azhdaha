import subprocess

# Run the safe tests

output_safe = (
    subprocess.run(
        ["just", "run", "./tests/compile_commands_safe.json", "--do-not-report"],
        capture_output=True,
        text=True,
    )
    .stdout.strip()
    .split("\n")
)

problematic_safe = list(filter(lambda entry: "problematic" in entry, output_safe))
false_positives = len(problematic_safe)

successful_safe = list(filter(lambda entry: "successfully" in entry, output_safe))
true_negative = len(successful_safe)

# Run the unsafe tests

output_unsafe = (
    subprocess.run(
        ["just", "run", "./tests/compile_commands_unsafe.json", "--do-not-report"],
        capture_output=True,
        text=True,
    )
    .stdout.strip()
    .split("\n")
)

problematic_unsafe = list(filter(lambda entry: "problematic" in entry, output_unsafe))
true_positives = len(problematic_unsafe)

successful_unsafe = list(filter(lambda entry: "successfully" in entry, output_unsafe))
false_negative = len(successful_unsafe)

# Report the results

print(f"false_positives = {false_positives}")
print(f"true_negative = {true_negative}")
print(f"true_positives = {true_positives}")
print(f"false_negative = {false_negative}")

precision = true_positives / (true_positives + false_positives)
recall = true_positives / (true_positives + false_negative)

print()
print(f"precision = {precision}")
print(f"recall = {recall}")
