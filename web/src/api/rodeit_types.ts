/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


export interface paths {
  "/bucket_list/": {
    /**
     * Get Coasters and Bucket List Counts
     * @description Get all coasters and the amount of bucket lists they are in
     */
    get: {
      responses: {
        /** @description List of Coasters and Bucket List Counts */
        200: {
          content: {
            "application/json": components["schemas"]["CoasterIdAndCount"][];
          };
        };
        /** @description Bucket List not found */
        404: {
          content: never;
        };
      };
    };
  };
  "/bucket_list/{user_id}": {
    /**
     * Get Coasters in a Bucket List
     * @description Get all coasters in a bucket list
     */
    get: {
      responses: {
        /** @description List of Coasters in Bucket List */
        200: {
          content: {
            "application/json": number[];
          };
        };
        /** @description Bucket List not found */
        404: {
          content: never;
        };
      };
    };
    /**
     * Set Coasters in a Bucket List
     * @description Set all coasters in a bucket list
     */
    put: {
      requestBody: {
        content: {
          "application/json": number[];
        };
      };
      responses: {
        /** @description Set Bucket List */
        200: {
          content: never;
        };
        /** @description Unauthorized */
        401: {
          content: never;
        };
        /** @description Bucket List not found */
        404: {
          content: never;
        };
      };
    };
    /**
     * Add a Coaster
     * @description Add a coaster to a bucket list
     */
    post: {
      requestBody: {
        content: {
          "application/json": number;
        };
      };
      responses: {
        /** @description Added to Bucket List */
        200: {
          content: never;
        };
        /** @description Unauthorized */
        401: {
          content: never;
        };
        /** @description Bucket List not found */
        404: {
          content: never;
        };
      };
    };
  };
  "/bucket_list/{user_id}/{index}": {
    /**
     * Get a Coaster by index
     * @description Get a coaster at a given index in a bucket list
     */
    get: {
      responses: {
        /** @description Coaster in Bucket List */
        200: {
          content: {
            "application/json": number;
          };
        };
        /** @description Bucket List not found or index out of bounds */
        404: {
          content: never;
        };
      };
    };
    /**
     * Insert a Coaster by index
     * @description Insert a coaster at a given index into a bucket list
     */
    post: {
      requestBody: {
        content: {
          "application/json": number;
        };
      };
      responses: {
        /** @description Inserted into Bucket List */
        200: {
          content: never;
        };
        /** @description Unauthorized */
        401: {
          content: never;
        };
        /** @description Bucket List not found */
        404: {
          content: never;
        };
      };
    };
    /**
     * Delete a Coaster by index
     * @description Delete a coaster at a given index from a bucket list
     */
    delete: {
      responses: {
        /** @description Deleted from Bucket List */
        200: {
          content: never;
        };
        /** @description Unauthorized */
        401: {
          content: never;
        };
        /** @description Bucket List not found or index out of bounds */
        404: {
          content: never;
        };
      };
    };
  };
  "/records/record": {
    /**
     * Submit a Record
     * @description Submit a new coaster to a user
     */
    post: operations["record.submit"];
  };
  "/records/record/{record_id}": {
    /** Get a Record by ID */
    get: operations["record.get_by_id"];
  };
  "/records/user/{user_id}": {
    /**
     * Get Records by User
     * @description Get all records of a user
     */
    get: operations["record.get_by_user"];
  };
  "/records/rollercoaster/{rollercoaster_id}": {
    /**
     * Get Records by Coaster
     * @description Get records of all users on a coaster
     */
    get: operations["record.get_by_rollercoaster"];
  };
  "/records/records": {
    /**
     * Get all Records
     * @description Get all records with pagination and sorting
     */
    get: operations["records.get"];
  };
  "/records/records/aggregate": {
    /**
     * Aggregate how often a roller coaster has been ridden
     * @description Aggregate how often a roller coaster has been ridden
     */
    get: operations["records.aggregated"];
  };
  "/users/getAllUsers": {
    /**
     * Get all Users
     * @description Get all Users
     */
    get: operations["get_all_users"];
  };
  "/users/login": {
    /**
     * Login
     * @description Login
     *
     * Login with username and password and return JWT
     */
    post: operations["login"];
  };
  "/users/register": {
    /**
     * Register
     * @description Register
     *
     * Register a new user
     */
    post: operations["register_user"];
  };
  "/users/user/:user_id": {
    /**
     * Delete a User by ID
     * @description Delete a User by ID
     */
    delete: operations["delete_user"];
    /**
     * Update a User by ID
     * @description Update a User by ID
     */
    patch: operations["patch_user"];
  };
  "/users/user/:userid": {
    /**
     * Get a User by ID
     * @description Get a User by ID
     */
    get: operations["get_user_by_id"];
  };
}

export type webhooks = Record<string, never>;

export interface components {
  schemas: {
    CoasterIdAndCount: {
      /** Format: uint64 */
      coaster_id: number;
      /** Format: uint32 */
      count: number;
    };
    Record: {
      user_id: number;
      rollercoaster_id: number;
    };
    LoginRequest: {
      password: string;
      username: string;
    };
    LoginResponse: {
      token: string;
    };
    User: {
      /** Format: int64 */
      id?: number | null;
      name: string;
      password: string;
      surname: string;
      username: string;
    };
  };
  responses: never;
  parameters: {
    record_id: number;
    user_id: number;
    rollercoaster_id: number;
    /** @description The numbers of items to return */
    query_limit?: number;
    /** @description sort results by key */
    query_sort?: string;
    query_start?: string;
    query_end?: string;
  };
  requestBodies: never;
  headers: never;
  pathItems: never;
}

export type $defs = Record<string, never>;

export type external = Record<string, never>;

export interface operations {

  /**
   * Submit a Record
   * @description Submit a new coaster to a user
   */
  "record.submit": {
    requestBody: {
      content: {
        "application/json": components["schemas"]["Record"];
      };
    };
    responses: {
      201: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
    };
  };
  /** Get a Record by ID */
  "record.get_by_id": {
    parameters: {
      path: {
        record_id: components["parameters"]["record_id"];
      };
    };
    responses: {
      200: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
      /** @description Not found */
      404: {
        content: never;
      };
    };
  };
  /**
   * Get Records by User
   * @description Get all records of a user
   */
  "record.get_by_user": {
    parameters: {
      path: {
        user_id: components["parameters"]["user_id"];
      };
    };
    responses: {
      200: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
      /** @description Not found */
      404: {
        content: never;
      };
    };
  };
  /**
   * Get Records by Coaster
   * @description Get records of all users on a coaster
   */
  "record.get_by_rollercoaster": {
    parameters: {
      path: {
        rollercoaster_id: components["parameters"]["rollercoaster_id"];
      };
    };
    responses: {
      200: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
      /** @description Not found */
      404: {
        content: never;
      };
    };
  };
  /**
   * Get all Records
   * @description Get all records with pagination and sorting
   */
  "records.get": {
    parameters: {
      query?: {
        limit?: components["parameters"]["query_limit"];
        sort?: components["parameters"]["query_sort"];
        start?: components["parameters"]["query_start"];
        end?: components["parameters"]["query_end"];
      };
    };
    responses: {
      200: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
    };
  };
  /**
   * Aggregate how often a roller coaster has been ridden
   * @description Aggregate how often a roller coaster has been ridden
   */
  "records.aggregated": {
    responses: {
      200: {
        content: never;
      };
      /** @description Access token is missing or invalid */
      401: {
        content: never;
      };
    };
  };
  /**
   * Get all Users
   * @description Get all Users
   */
  get_all_users: {
    responses: {
      /** @description operation successful */
      200: {
        content: {
          "application/json": components["schemas"]["User"][];
        };
      };
      /** @description something went wrong */
      500: {
        content: never;
      };
    };
  };
  /**
   * Login
   * @description Login
   *
   * Login with username and password and return JWT
   */
  login: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["LoginRequest"];
      };
    };
    responses: {
      /** @description operation successful */
      200: {
        content: {
          "application/json": components["schemas"]["LoginResponse"];
        };
      };
      /** @description user not found and/or password not valid */
      401: {
        content: never;
      };
    };
  };
  /**
   * Register
   * @description Register
   *
   * Register a new user
   */
  register_user: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["User"];
      };
    };
    responses: {
      /** @description operation successful */
      200: {
        content: {
          "application/json": components["schemas"]["User"];
        };
      };
      /** @description user already exists */
      400: {
        content: never;
      };
    };
  };
  /**
   * Delete a User by ID
   * @description Delete a User by ID
   */
  delete_user: {
    responses: {
      /** @description operation successful */
      200: {
        content: never;
      };
      /** @description not permitted to delete this user */
      401: {
        content: never;
      };
      /** @description user not found */
      404: {
        content: never;
      };
    };
  };
  /**
   * Update a User by ID
   * @description Update a User by ID
   */
  patch_user: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["UpdateUser"];
      };
    };
    responses: {
      /** @description operation successful */
      200: {
        content: never;
      };
      /** @description not permitted to change this user */
      401: {
        content: never;
      };
      /** @description user not found */
      404: {
        content: never;
      };
    };
  };
  /**
   * Get a User by ID
   * @description Get a User by ID
   */
  get_user_by_id: {
    responses: {
      /** @description operation successful */
      200: {
        content: never;
      };
      /** @description user not found */
      404: {
        content: never;
      };
    };
  };
}
