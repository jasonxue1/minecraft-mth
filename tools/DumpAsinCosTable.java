import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/*
 * Regenerate command:
 * javac tools/DumpAsinCosTable.java && java -cp tools DumpAsinCosTable src/asin_tab.rs src/cos_tab.rs
 */
public final class DumpAsinCosTable {
    private static final int TABLE_SIZE = 257;

    private DumpAsinCosTable() {
    }

    public static void main(String[] args) throws IOException {
        Path asinOut = Path.of(args.length > 0 ? args[0] : "src/asin_tab.rs");
        Path cosOut = Path.of(args.length > 1 ? args[1] : "src/cos_tab.rs");

        double[] asinTab = new double[TABLE_SIZE];
        double[] cosTab = new double[TABLE_SIZE];

        for (int ind = 0; ind < TABLE_SIZE; ind++) {
            double v = ind / 256.0;
            double asinv = Math.asin(v);
            cosTab[ind] = Math.cos(asinv);
            asinTab[ind] = asinv;
        }

        writeTable(asinOut, "ASIN_TAB", asinTab);
        writeTable(cosOut, "COS_TAB", cosTab);
    }

    private static void writeTable(Path output, String constName, double[] values) throws IOException {
        StringBuilder sb = new StringBuilder(16_384);
        sb.append("pub static ").append(constName).append(": [f64; ").append(values.length).append("] = [\n");

        for (int i = 0; i < values.length; i++) {
            long bits = Double.doubleToRawLongBits(values[i]);
            sb.append("    f64::from_bits(0x")
              .append(String.format("%016X", bits))
              .append("),");
            if (i % 4 == 3) {
                sb.append('\n');
            } else {
                sb.append(' ');
            }
        }

        if (values.length % 4 != 0) {
            sb.append('\n');
        }

        sb.append("];\n");
        Files.writeString(output, sb.toString(), StandardCharsets.UTF_8);
    }
}
