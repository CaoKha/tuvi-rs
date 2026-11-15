//! The card text, from `src/content/` of the old app (branch `main`):
//! `destiny.ts`. The English text is new.

/// Lá Bài Bản Mệnh: `(chính tinh of cung Mệnh, archetype and mantra in
/// English, then in Vietnamese)`. The 39 sets of chính tinh that cung Mệnh
/// can hold, in any order. The empty set is vô chính diệu.
#[rustfmt::skip]
pub(super) const DESTINY: [(&[&str], &str, &str, &str, &str); 39] = [
    (&[], "The Self-Shaper", "An empty Mệnh is not a void — it is open sky where you decide who you are.", "Người Tự Định Hình", "Mệnh trống không phải khoảng trống — đó là khoảng trời để ta tự định nghĩa mình là ai."),
    (&["Tử Vi"], "The Sovereign", "The greatest power is the power to rule yourself.", "Bậc Đế Vương", "Quyền lực lớn nhất là quyền làm chủ chính mình."),
    (&["Thiên Phủ"], "The Keeper of the Vault", "A full vault matters less than a steady heart — you are the one everyone leans on.", "Người Giữ Kho", "Đầy kho không bằng vững lòng — ta là chỗ cả nhà dựa vào."),
    (&["Thất Sát"], "The Vanguard General", "I walk ahead, so the ground behind me becomes a road.", "Vị Tướng Tiên Phong", "Ta đi trước, để đường sau lưng ta trở thành đường."),
    (&["Phá Quân"], "The Breaker of Ranks", "Breaking is not an ending; it is how I begin again.", "Người Phá Trận", "Phá vỡ không phải kết thúc; đó là cách ta bắt đầu lại từ đầu."),
    (&["Tham Lang"], "The Seeker of Desire", "Desire is a flame — I learn to hold it, not to be held by it.", "Người Khát Khao", "Khát khao là ngọn lửa — ta học cách giữ lửa, không để lửa giữ ta."),
    (&["Thái Dương"], "The Light-Bringer", "The sun asks no one's leave to shine.", "Người Soi Sáng", "Mặt trời không xin phép ai để tỏa sáng."),
    (&["Thái Âm"], "The Moon Keeper", "Gentleness is a strength that needs no proof.", "Người Giữ Trăng", "Dịu dàng là một sức mạnh không cần chứng minh."),
    (&["Vũ Khúc"], "The Metalsmith", "I trust what can be counted, and make it worth trusting.", "Người Rèn Kim", "Ta tin vào điều đếm được, và làm cho điều đó đáng tin."),
    (&["Liêm Trinh"], "The Flame Keeper", "My discipline and my passion do not fight — they burn together.", "Người Giữ Lửa", "Kỷ luật và đam mê trong ta không đánh nhau — chúng cùng cháy."),
    (&["Thiên Cơ"], "The Strategist", "Seeing the road before it appears is my gift.", "Người Vẽ Cơ", "Nhìn thấy con đường trước khi nó hiện ra là món quà của ta."),
    (&["Thiên Lương"], "The Protector", "Shielding others does not make me smaller; it makes me taller.", "Người Che Chở", "Che chở người khác không làm ta nhỏ đi; nó làm ta cao hơn."),
    (&["Thiên Tướng"], "The Seal Bearer", "My word weighs like a seal — that is why people trust it.", "Người Mang Ấn", "Lời hứa của ta nặng như ấn triện — đó là lý do người ta tin."),
    (&["Thiên Đồng"], "The Keeper of Blessings", "I choose gentleness not from weakness, but because I know enough is enough.", "Người Giữ Phúc", "Ta chọn hiền hoà không phải vì yếu, mà vì biết đủ."),
    (&["Cự Môn"], "The Opener of Doors", "My words are a door — I learn to open it at the right moment.", "Người Mở Cánh Cửa", "Lời nói của ta là một cánh cửa — ta học cách mở đúng lúc."),
    (&["Thiên Phủ", "Tử Vi"], "The Monarch", "I need not seize the throne — I lay the foundation, and the throne comes.", "Bậc Quân Chủ", "Ta không cần giành ngôi — ta xây nền, rồi ngôi tự đến."),
    (&["Tham Lang", "Tử Vi"], "The Conqueror", "Wanting much is no flaw; I turn wanting much into doing much.", "Kẻ Chinh Phục", "Muốn nhiều không phải lỗi; ta biến muốn nhiều thành làm được nhiều."),
    (&["Thiên Tướng", "Tử Vi"], "The Seal of Authority", "My standing is sealed by deeds, not by words.", "Bậc Quyền Ấn", "Uy tín của ta được đóng dấu bằng hành động, không bằng lời."),
    (&["Thất Sát", "Tử Vi"], "The Founder", "I command myself first — the world follows after.", "Bậc Khai Quốc", "Ta ra lệnh cho bản thân trước — thế giới nghe theo sau."),
    (&["Phá Quân", "Tử Vi"], "The Re-Founder", "I dare to break even what I built, to build it right again.", "Người Khai Hoàng", "Ta dám phá cả những gì mình từng xây, để xây lại cho đúng."),
    (&["Phá Quân", "Vũ Khúc"], "The Fortune Changer", "I dare to stake both capital and standing for a new fortune.", "Người Đổi Vận", "Ta dám bỏ cả vốn lẫn vị thế để đổi lấy vận mới."),
    (&["Thiên Phủ", "Vũ Khúc"], "The Treasurer", "Money comes and stays with me, because I value every coin.", "Bậc Trữ Tài", "Tiền đến rồi ở lại với ta, vì ta coi trọng từng đồng một."),
    (&["Tham Lang", "Vũ Khúc"], "The Fortune Hunter", "I hunt chances with both hunger and nerve.", "Người Săn Tài", "Ta săn cơ hội bằng cả khát khao lẫn bản lĩnh."),
    (&["Thất Sát", "Vũ Khúc"], "The Blade of Metal", "My decisions are fast but never careless — the blade is sharp because it passed through the forge.", "Vị Tướng Kiếm Kim", "Quyết định của ta nhanh nhưng không hời — kiếm sắc vì đã qua lò rèn."),
    (&["Thiên Tướng", "Vũ Khúc"], "The Appraiser", "I know the price of money, and the price of keeping my word.", "Người Định Giá", "Ta biết giá của tiền, và biết giá của chữ tín."),
    (&["Liêm Trinh", "Thiên Phủ"], "The Guardian of the Citadel", "Inside the walls a treasury, around them a fire — I both keep and defend.", "Người Giữ Thành", "Trong thành có kho, quanh thành có lửa — ta vừa giữ, vừa bảo vệ."),
    (&["Liêm Trinh", "Thất Sát"], "The Steadfast General", "I am hard on my goals, and harder still on myself.", "Vị Kiên Tướng", "Ta cứng với mục tiêu, và cứng hơn nữa với chính mình."),
    (&["Liêm Trinh", "Thiên Tướng"], "The Upright Seal", "I live so straight that others use me as their measure.", "Bậc Chính Ấn", "Ta sống ngay thẳng đến mức người khác lấy ta làm thước."),
    (&["Liêm Trinh", "Tham Lang"], "The Passionate One", "My passion does not scatter — it gathers into a single point.", "Người Đam Mê", "Đam mê của ta không tản ra — nó hội tụ thành một mũi nhọn."),
    (&["Liêm Trinh", "Phá Quân"], "The Breaker-Builder", "I break old rules, then set new ones, stricter than before.", "Người Phá Lập", "Ta phá luật cũ rồi tự đặt luật mới, nghiêm khắc hơn."),
    (&["Thái Âm", "Thái Dương"], "The Twin Lights", "I carry both sun and moon — I know how to blaze and how to soothe.", "Người Hai Vầng Sáng", "Ta mang cả mặt trời lẫn mặt trăng — biết rực rỡ, biết dịu dàng."),
    (&["Thái Dương", "Thiên Lương"], "The Warm Sun", "I shine to warm, not to burn.", "Người Nắng Ấm", "Ta tỏa sáng để sưởi ấm, không phải để thiêu đốt."),
    (&["Cự Môn", "Thái Dương"], "The Voice of Light", "My words are like sunlight — direct, clear, and they bring everything into view.", "Người Nói Ánh Sáng", "Lời ta như nắng — thẳng, rõ, và khiến mọi thứ lộ ra."),
    (&["Thái Âm", "Thiên Cơ"], "The Moonlit Mind", "My wisdom rises like the moon — quiet, yet it lights everything.", "Người Trăng Trí", "Trí tuệ của ta lên như trăng — lặng lẽ mà soi khắp."),
    (&["Thái Âm", "Thiên Đồng"], "The Gentle Water", "I am soft as water; and like water, nothing wears me down.", "Người Nước Hiền", "Ta mềm như nước; và như nước, không gì mài được ta."),
    (&["Cự Môn", "Thiên Đồng"], "The Kind Voice", "I speak a little less, so what I say is more worth hearing.", "Người Lời Hiền", "Ta nói ít hơn một chút, để lời nói ra đáng nghe hơn."),
    (&["Cự Môn", "Thiên Cơ"], "The Debater", "I see the gap in an argument as I see a crack in a wall.", "Người Biện Cơ", "Ta thấy kẽ hở trong lập luận như thấy kẽ hở trong bức tường."),
    (&["Thiên Cơ", "Thiên Lương"], "The Virtuous Strategist", "Cleverness used to help others is cleverness that lasts.", "Người Mưu Đức", "Mưu trí dùng để giúp người mới là mưu trí bền."),
    (&["Thiên Đồng", "Thiên Lương"], "The Hearth of Blessings", "My good fortune is not loud — it is a hearth that stays warm.", "Người Phúc Ấm", "Phúc của ta không ồn ào — nó như bếp lửa, ấm lâu."),
];
