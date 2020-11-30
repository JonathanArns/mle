# Matrikel Nr.: 1811609
import sys
import time
import math
import random
try:
    import numpy as np
except:
    print("ERROR: Numpy not installed properly.")
    sys.exit()
try:
    from OpenGL.GLUT import *
    from OpenGL.GL import *
    from OpenGL.GLU import *
except:
    print("ERROR: PyOpenGL not installed properly.")
    sys.exit()

'''
INSTALLATION:
-----------------------------------------
unter anaconda (python 3.6):
    conda install numpy
    conda install freeglut
    conda install pyopengl
    conda install pyopengl-accelerate

(bei fehlenden Bibliotheken googeln)

Ausführung:
    start anaconda prompt
    navigiere in den Game Ordner
    tippe: python A5.py
-----------------------------------------
'''

class GameGL(object):
    config = None
    def __init__(self, config = None):
        self.config = config
    '''
    Is needed for the OpenGL-Library because standard strings are not allowed.
    '''
    def toCString(self, string):
        return bytes(string, "ascii")

def get_state(xBall, yBall, xSchlaeger, xV, yV):
    if xV == -1:
        xv = 0
    else:
        xv = 1
    if yV == -1:
        yv = 0
    else:
        yv = 1
    return ((((xBall * 9) + yBall) * 7 + xSchlaeger) * 2 + xv) * 2 + yv 
    
class BasicGame(GameGL):

    windowName = "PingPong"
    # 30px
    pixelSize = 30

    xBall      = 5
    yBall      = 6
    xSchlaeger = 5
    xV         = 1
    yV         = 1
    score      = 0

    learn_rate = 0.5
    discount = 0.8
    
    def __init__(self, name, width = 300, height = 270):
        super
        self.windowName = name
        self.width      = width
        self.height     = height

    def keyboard(self, key, x, y):
        # ESC = \x1w
        if key == b'\x1b':
            sys.exit(0)
    
    def init(self):
        self.Q = [[random.uniform(0, 1) for _ in range(2)] for _ in range(get_state(9,8,6,1,1))]
        self.s = get_state(self.xBall, self.yBall, self.xSchlaeger, self.xV, self.yV)

    def display(self):
        # clear the screen
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        # reset position
        glLoadIdentity()
        glViewport(0, 0, self.width, self.height)
        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        glOrtho(0.0, self.width, 0.0, self.height, 0.0, 1.0)
        glMatrixMode (GL_MODELVIEW)
        glLoadIdentity()



        a = self.Q[self.s].index(max(self.Q[self.s])) # choose action a

        if a == 0:
            self.xSchlaeger -= 1
        else:
            self.xSchlaeger += 1
        # don't allow puncher to leave the pitch
        if self.xSchlaeger < 0:
            self.xSchlaeger = 0
        if self.xSchlaeger > 6:
            self.xSchlaeger = 6
        

        self.xBall += self.xV
        self.yBall += self.yV
        # change direction of ball if it's at wall
        if (self.xBall > 8 or self.xBall < 1):
            self.xV = -self.xV
        if (self.yBall > 7 or self.yBall < 1):
            self.yV = -self.yV
        # check whether ball on bottom line
        if self.yBall == 0:
            # check whether ball is at position of player
            if (self.xSchlaeger == self.xBall 
                or self.xSchlaeger == self.xBall -1
                or self.xSchlaeger == self.xBall -2
                or self.xSchlaeger == self.xBall -3):
                print("positive reward")
                r = 1
            else:
                print("negative reward")
                r = -1
        else:
            r = 0


        s_next = get_state(self.xBall, self.yBall, self.xSchlaeger, self.xV, self.yV)
        self.Q[self.s][a] += self.learn_rate * (r + self.discount * max(self.Q[s_next]) - self.Q[self.s][a])
        self.s = s_next


        
        # repaint
        self.drawBall()
        self.drawComputer()
        
        # timeout
        time.sleep(0.001)
        
        glutSwapBuffers()
    
    def start(self):
        glutInit()
        glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_ALPHA | GLUT_DEPTH)
        glutInitWindowSize(self.width, self.height)
        glutInitWindowPosition(100, 100)
        glutCreateWindow(self.toCString(self.windowName))
        self.init()
        glutDisplayFunc(self.display)
        glutReshapeFunc(self.onResize)
        glutIdleFunc(self.display)
        glutKeyboardFunc(self.keyboard)
        glutMainLoop() 
    
    def updateSize(self):
        self.width  = glutGet(GLUT_WINDOW_WIDTH)
        self.height = glutGet(GLUT_WINDOW_HEIGHT)
    
    def onResize(self, width, height):
        self.width  = width
        self.height = height
    
    def drawBall(self, width = 1, height = 1, x = 5, y = 6, color = (0.0, 1.0, 0.0)):
        x = self.xBall
        y = self.yBall
        xPos = x * self.pixelSize
        yPos = y * self.pixelSize
        # set color
        glColor3f(color[0], color[1], color[2])
        # start drawing a rectangle
        glBegin(GL_QUADS)
        # bottom left point
        glVertex2f(xPos, yPos)
        # bottom right point
        glVertex2f(xPos + (self.pixelSize * width), yPos)
        # top right point
        glVertex2f(xPos + (self.pixelSize * width), yPos + (self.pixelSize * height))
        # top left point
        glVertex2f(xPos, yPos + (self.pixelSize * height))
        glEnd()
    
    def drawComputer(self, width = 4, height = 1, x = 0, y = 0, color = (1.0, 0.0, 0.0)):
        x = self.xSchlaeger
        xPos = x * self.pixelSize
        # set a bit away from bottom
        yPos = y * self.pixelSize# + (self.pixelSize * height / 2)
        # set color
        glColor3f(color[0], color[1], color[2])
        # start drawing a rectangle
        glBegin(GL_QUADS)
        # bottom left point
        glVertex2f(xPos, yPos)
        # bottom right point
        glVertex2f(xPos + (self.pixelSize * width), yPos)
        # top right point
        glVertex2f(xPos + (self.pixelSize * width), yPos + (self.pixelSize * height / 4))
        # top left point
        glVertex2f(xPos, yPos + (self.pixelSize * height / 4))
        glEnd()

if __name__ == '__main__':
    game = BasicGame("PingPong")
    game.start()