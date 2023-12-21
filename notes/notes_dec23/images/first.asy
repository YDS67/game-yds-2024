defaultpen(fontsize(10pt));

settings.render = 16;
unitsize(5cm);

int ntiles = 10;
real wtiles = 1.0/ntiles;
pair player_pos = (0.37, 0.33);
real pa = 1;
pair player_dir = player_pos+2.7wtiles*(cos(pa), sin(pa));
real pFOV = 0.8;
pair player_left = player_pos+2wtiles*(cos(pa+pFOV), sin(pa+pFOV));
pair player_right = player_pos+2wtiles*(cos(pa-pFOV), sin(pa-pFOV));
path player_view = player_pos -- player_left -- arc(player_pos, 2wtiles, degrees(pa+pFOV), degrees(pa-pFOV)) -- player_right -- cycle;

label("Game world, top view", (0.5, 1.1), align=N);

int i = 6;
int j = 5;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), gray);
int i = 6;
int j = 6;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), gray);
int i = 7;
int j = 5;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), gray);
int i = 6;
int j = 7;
fill(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), gray);


for(int i=0; i < ntiles; ++i) {
for(int j=0; j < ntiles; ++j) {
    draw(box((i*wtiles,j*wtiles),((i+1)*wtiles,(j+1)*wtiles)), darkgray);
}
}

draw((0,0) -- (1.1,0), arrow = Arrow);
draw((0,0) -- (0,1.1), arrow = Arrow);
label("$x$", (1.1,0), align=E);
label("$y$", (0,1.1), align=N);
label("$0$", (0,0), align=SW);
label("$1$", (0.1,0), align=S);
label("$1$", (0,0.1), align=W);

fill(player_view, blue+opacity(0.2));
draw(player_view, blue);
draw(player_pos -- (player_pos.x,0), red+dashed);
draw(player_pos -- (0,player_pos.y), red+dashed);
label("$x_p$", (player_pos.x,0), align=S);
label("$y_p$", (0,player_pos.y), align=W);
label("P", player_pos, align=2S+2W, Fill(white));
label("FOV horizontal", player_pos, align=S+2E, fontsize(8pt), Fill(white));
draw(player_pos -- player_dir, blue, Arrow);
dot(player_pos, red);