import pygame
import random
from enum import Enum

# Initialize Pygame
pygame.init()

# Set up the display
DISPLAI_WIDTH = 1280
DISPLAY_HEIGHT = 720
game_display = pygame.display.set_mode((DISPLAI_WIDTH, DISPLAY_HEIGHT))
pygame.display.set_caption("T-Rex Game")

# Colors
black = (0, 0, 0)
white = (255, 255, 255)

ressource_folder = "../ressources/"

# Load the images
dinosaur_img = pygame.image.load(ressource_folder + "humain.png")
cactus_img = pygame.image.load(ressource_folder + "cactus.png")
rock_image = pygame.image.load(ressource_folder + "rock.png")
pterodactyl_img = pygame.image.load(ressource_folder + "pterodactyle.png")

background_img = pygame.image.load(ressource_folder + "landscape.png")
background_img = pygame.transform.scale(background_img, (DISPLAI_WIDTH, DISPLAY_HEIGHT))


# Define the dinosaur properties
DINAUSOR_WIDTH = 60
DINAUSOR_HEIGHT = 90
dinosaur_x = 100
dinosaur_y = DISPLAY_HEIGHT - DINAUSOR_HEIGHT - 10
dinosaur_y_change = 0
dinosaur_img = pygame.transform.scale(dinosaur_img, (DINAUSOR_WIDTH, DINAUSOR_HEIGHT))
dinosaur_img = pygame.transform.flip(dinosaur_img, False, True)

# -------------------------- obstacle --------------------------
obstacle_spawn_timer = 0
obstacle_spawn_interval = 1500  # milliseconds (2 seconds)

obstacle_x_change = -5

# Define obstacle types as an enum
class ObstacleType(Enum):
    CACTUS = 1
    ROCK = 2
    PTERODACTYL = 3

# Define the cactus properties
cactus_width = 40
cactus_height = 60
cactus_x = DISPLAI_WIDTH
cactus_y = DISPLAY_HEIGHT - cactus_height - 10
cactus_img = pygame.transform.scale(cactus_img, (cactus_width, cactus_height))
cactus_img = pygame.transform.flip(cactus_img, False, True)

# Define the rock properties
rock_width = 40
rock_height = 40
rock_x = DISPLAI_WIDTH
rock_y = DISPLAY_HEIGHT - rock_height - 10
rock_img = pygame.transform.scale(rock_image, (rock_width, rock_height))
rock_img = pygame.transform.flip(rock_img, False, True)

# define the pterodactyl properties
pterodactyl_width = 40
pterodactyl_height = 40
pterodactyl_x = DISPLAI_WIDTH
pterodactyl_y = DISPLAY_HEIGHT - pterodactyl_height - DINAUSOR_HEIGHT - 10 # - DINAUSOR_HEIGHT to make the pterodactyl fly in the air
pterodactyl_img = pygame.transform.scale(pterodactyl_img, (pterodactyl_width, pterodactyl_height))
pterodactyl_img = pygame.transform.flip(pterodactyl_img, False, True)

# -------------------------- game loop --------------------------

def check_collision(rect1, rect2):
    """
    Check collision between two rectangles/squares.
    Each rectangle is defined by (x, y, width, height).
    Returns True if there is a collision, False otherwise.
    """
    x1, y1, w1, h1 = rect1
    x2, y2, w2, h2 = rect2

    return (x1 < x2 + w2 and
        x1 + w1 > x2 and
        y1 < y2 + h2 and
        y1 + h1 > y2)

# Create an empty list to store obstacles
obstacles = []

# Game loop
clock = pygame.time.Clock()
game_over = False

while not game_over:
    # -------------------------- event --------------------------
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            game_over = True
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_SPACE:
                dinosaur_y_change = -10
        if event.type == pygame.KEYUP:
            if event.key == pygame.K_SPACE:
                dinosaur_y_change = 10
    
    # -------------------------- update --------------------------
    # Move the dinosaur
    dinosaur_y += dinosaur_y_change
    # Check for collision with the ground
    if dinosaur_y >= DISPLAY_HEIGHT - DINAUSOR_HEIGHT - 10:
        dinosaur_y = DISPLAY_HEIGHT - DINAUSOR_HEIGHT - 10

    # Move and remove obstacles
    for obstacle in obstacles:
        obstacle['x'] += obstacle_x_change
        if obstacle['x'] + obstacle['width'] < 0:
            obstacles.remove(obstacle)

    # -------------------------- new obstacle --------------------------
    # spawn obstacles
    obstacle_spawn_timer += clock.get_rawtime()
    if obstacle_spawn_timer >= obstacle_spawn_interval:
        obstacle_type = random.choice(list(ObstacleType))
        if obstacle_type == ObstacleType.CACTUS:
            obstacle_img = cactus_img
            obstacle_height = cactus_height
            obstacle_width = cactus_width
            obstacle_y = cactus_y
        elif obstacle_type == ObstacleType.ROCK:
            obstacle_img = rock_img
            obstacle_height = rock_height
            obstacle_width = rock_width
            obstacle_y = rock_y
        elif obstacle_type == ObstacleType.PTERODACTYL:
            obstacle_img = pterodactyl_img
            obstacle_height = pterodactyl_height
            obstacle_width = pterodactyl_width
            obstacle_y = pterodactyl_y

        new_obstacle = {
            'x': DISPLAI_WIDTH,
            'y': obstacle_y,
            'type': obstacle_type,
            'width': obstacle_width,
            'height': obstacle_height,
            'img': obstacle_img
        }
        obstacles.append(new_obstacle)
        obstacle_spawn_timer = 0


    # -------------------------- collision --------------------------

    # Check for collision with the cactus
    for obstacle in obstacles:
        obstacle_rect = (obstacle['x'], obstacle['y'], obstacle["width"], obstacle["height"])
        dinosaur_rect = (dinosaur_x, dinosaur_y, DINAUSOR_WIDTH, DINAUSOR_HEIGHT)
       
        if check_collision(obstacle_rect, dinosaur_rect):
            game_over = True

    # -------------------------- draw --------------------------

    game_display.fill(white)
    game_display.blit(background_img, (0, 0))
    game_display.blit(dinosaur_img, (dinosaur_x, dinosaur_y))
    for obstacle in obstacles:
        obstacle_img = obstacle['img']
        obstacle_x = obstacle['x']
        obstacle_y = obstacle['y']
        game_display.blit(obstacle_img, (obstacle_x, obstacle_y))

    # ------------------------- continue -------------------------

    # Update the display
    pygame.display.update()
    clock.tick(60)

# Quit Pygame
pygame.quit()


