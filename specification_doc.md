# Specification stuff for BattleBrawler
Shit gets complicated fast XD


## Player Movement  

Player movement is divided into two parts:
- Passive Movement via e.g., gravity
- Active Movement via player input

Player Movement acts upon the player position x [pixels], y [pixels]. Changes to the player position can be modelled as the player having a velocity. Velocity is defined as [distance/time], in our case [pixels/second], and therefore independent of the frame rate the gameloop runs in. Furthermore acceleration is defined as [pixels/seconds^2], this means we apply this acceleration every second to the velocity in fps many steps.

Furthermore we implement a maximum movementspeed that can only be surpassed by 

### Passive or involuntary movement
Currently there is one one form of passive movement:
- Gravity
- Drag
  
#### Gravity
Gravity applies a constant downwards acceleration to the player.  
The gravity constant specifies how many more pixels per second the player should move downwards, towards positive y.

### Drag
Drag is like gravity, but only for action_velocity, and it needs to respect negative and positive directions.

### Active Movement
Active Movement depends on player input. Currently we support TODO: many actions.

- Move right (->)
- Move left (<-)
- Jump (^)
- Dive (v)
- Right Dash (R-STRG + ->)
- Left Dash (R-STRG + <-)

Movement in the game should feel snappy, therefore we do not accelerate the player, we directly increase his velocity. This only applies to grounded movement, we do accelerate when airbone, although the acceleration is high.

#### Move right and left
Sets the player's basic_velocity to max_basic_velocity if the player is grounded. If the player is grounded no acceleration can be applied.
If the player is airbone, he will accelerate airbone_acceleration amount instead. The players horizontal basic_velocity is limited by max_basic_velocity.

basic_velocity: Box<(f32,f32)>
action_velocity: Box<(f32,f32)> 
player_velocity: Box<(f32,f32)> = basic_velocity + action_velocity
player_position: Box<(f32,f32)> += player_velocity;

#### Jump
Adds a constant upwards velocity to the action_velocity, we implement a double jump. The first jump can only be executed if the player is grounded. The second jump can only be executed when the player is airbone, it sets the downwards action_velocity to zero and adds second_jump to the upwards velocity.

#### Dive
Dive sets the upwards action_velocity of the player to zero and adds a constant downwards action_velocity, it can only be used when airbone and has only one cast. the cast refreshes when grounding.

#### Right and Left Dash
If the player is grounded, Dash sets the horizontal action_velocity to dash_velocity for grounded_dash_duration [seconds]. You can only dash on the ground when no longer dashing and no longer moving right or left aka if the total horizontal velocity is zero.
If the player is airbone, Dash sets the player's horizontal velocity to zero. And applies a constant horizontal dash_velocity to the action_velocity. The player can dash again, if the vertical action_velocity is zero again.

## Movement Related constants
- gravity_acceleration = 30.
- drag_acceleration = 50.
- first_jump_velocity = 1000.
- second_jump_velocity = 700.
- jump_count = 0 (records how many times the player has jumped)
- dash_velocity = 1000.
- grounded_dash_duration = 0.3
- max_basic_velocity = 500. (max basic player movement speed)
- airbone_acceleration = 2000.
- player_velocity
- basic_velocity
- action_velocity



Player has orientation. Player has position. Player has grounded/airbone state


## Multithreading Engine

The game engine will be logically seperated into two parts:
1. Update
2. Render

The Update function loop runs on the "main" thread will constantly update the game state based on the previous game state and user interaction. Furthermore, it will produce a RenderState that contains all information the renderer needs to display the game state. The Update function has its own frequency called Ticks Per Second (TPS). Additionally, it has to consider the frequency as a scaling factor for the internal logic such as Physics, aka. objects move x pixel/second, regardeless of the effective TPS.

The Render function loop runs on a seperate thread, and *Interpolates* previous RenderStates to produce a time accurate prediction on what the game state would look like at the time of the Render execution. This requires the RenderState to be continuous to produce accurate results. Furthermore the render function can be unlocked to push as many frames as possible, or locked to a certain frame rate, and idle in between. The Render function has its own frequency called Frames Per Second (FPS).


The engine Supports additional Multithreading via a Thread pool, that can accept jobs from the two loops. This approach incurrs minimal overhead and is therefore preffered to dynamic thread creation.



### Visualization
The figure below visualizes The Thread organization.

```
]|[-----Update-----]|[-----Update-----]|[---Update---]...
[RenderState1]     [RenderState2]     [RenderState3]...
[Render1]|[Render2]|[Render3]|[Render4]|[Render5]|[Render6]...

Threadpool:
Thread2[-------------------------Jobs-------------------------
Thread3[-------------------------Jobs-------------------------
Thread4[-------------------------Jobs-------------------------

Render-RenderState dependency
Render1(RenderState0, RenderState1)
Render2(RenderState0, RenderState1)
Render3(RenderState1, RenderState2)
Render4(RenderState1, RenderState2)
Render5(RenderState2, RenderState3)
Render6(RenderState2, RenderState3)
```