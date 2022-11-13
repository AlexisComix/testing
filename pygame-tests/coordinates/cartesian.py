import pygame, sys

w = 500
h = 750

# --- The translation layer ---

# f(x, y )--- Takes in pygame coordinates and translates to cartesian
# This is so maths makes more sense with it
def to_cartesian(x: int, y: int):
    return x, h-y

# f'(x, y) --- Takes in cartesian coordinates and translates to pygame
# Cartesian values that have been calculated can be put in here
def to_pgcoords(x: int, y: int):
    return x, h-y

# --- Note how they are the same just abstracted ---

def main():
    win = pygame.display.set_mode((w,h))
    pygame.display.set_caption("Cartesian")

    running = True

    pygame.init()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
                pygame.quit()
                sys.exit()

        win.fill((0,0,0))
        pygame.draw.line(win, (255,0,0), to_pgcoords(0,0), to_pgcoords(0,h), width=2)
        # Adding 2 to the y value for visual representation
        # because pygame renders the pixels downwards
        pygame.draw.line(win, (0,255,0), to_pgcoords(0,2), to_pgcoords(w,2), width=2)

        pygame.display.flip()

if __name__ == "__main__":
    main()