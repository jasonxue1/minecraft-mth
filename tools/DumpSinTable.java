import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/*
 * Regenerate command:
 * javac tools/DumpSinTable.java && java -cp tools DumpSinTable src/sin_table.rs
 */
public final class DumpSinTable {
    private static final int TABLE_SIZE = 65536;
    private static final double SIN_SCALE = 10430.378350470453;

    private DumpSinTable() {
    }

    public static void main(String[] args) throws IOException {
        Path output = Path.of(args.length > 0 ? args[0] : "sin_table.rs");

        float[] sin = new float[TABLE_SIZE];
        for (int i = 0; i < sin.length; i++) {
            sin[i] = (float) Math.sin(i / SIN_SCALE);
        }

        StringBuilder sb = new StringBuilder(2_000_000);
        sb.append("pub static SIN: [f32; ").append(TABLE_SIZE).append("] = [\n");
        for (int i = 0; i < sin.length; i++) {
            int bits = Float.floatToRawIntBits(sin[i]);
            sb.append("    f32::from_bits(0x")
              .append(String.format("%08X", bits))
              .append("),");
            if (i % 8 == 7) {
                sb.append('\n');
            } else {
                sb.append(' ');
            }
        }
        if (sin.length % 8 != 0) {
            sb.append('\n');
        }
        sb.append("];\n");

        Files.writeString(output, sb.toString(), StandardCharsets.UTF_8);
    }
}
