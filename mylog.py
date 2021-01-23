import logging
import sys
filename = "log.log"
LOG_FORMAT = "%(created)f %(levelname)s %(message)s"
logging.basicConfig(filename=filename , filemode='w+', level=logging.INFO, format=LOG_FORMAT)