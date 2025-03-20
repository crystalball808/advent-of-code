const std = @import("std");
const debug = std.debug;

const NumberList = std.ArrayList(u32);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = try std.fs.cwd().readFileAlloc(allocator, "input.txt", 1024);
    defer allocator.free(input);

    var timer = try std.time.Timer.start();

    var first_column_numbers = NumberList.init(allocator);
    defer first_column_numbers.deinit();
    var second_column_numbers = NumberList.init(allocator);
    defer second_column_numbers.deinit();

    try prepCols(input, &first_column_numbers, &second_column_numbers);

    debug.assert(first_column_numbers.items.len == second_column_numbers.items.len);
    var sum: u32 = 0;
    var i: usize = 0;
    while (i < first_column_numbers.items.len) : (i += 1) {
        sum += absoluteDiff(first_column_numbers.items[i], second_column_numbers.items[i]);
    }

    const time = timer.lap();
    debug.print("Answer: {}\ntime: {} ns\n", .{ sum, time });
}

fn absoluteDiff(a: u32, b: u32) u32 {
    if (a > b) {
        return a - b;
    } else {
        return b - a;
    }
}

fn prepCols(input: []const u8, first: *NumberList, second: *NumberList) !void {
    var lines = std.mem.splitAny(u8, input, "\n");

    while (lines.next()) |line| {
        if (line.len == 0) break;

        var numbers_str = std.mem.splitSequence(u8, line, "   ");
        const first_number_str = numbers_str.next().?;
        const second_number_str = numbers_str.next().?;

        try first.append(try std.fmt.parseInt(u32, first_number_str, 10));
        try second.append(try std.fmt.parseInt(u32, second_number_str, 10));
    }
    std.mem.sort(u32, first.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, second.items, {}, comptime std.sort.asc(u32));
}
