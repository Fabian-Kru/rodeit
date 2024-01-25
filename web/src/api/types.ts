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
            "application/json": components["schemas"]["CoasterAndBucketListCount"][];
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
            "application/json": components["schemas"]["Coaster"][];
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
            "application/json": components["schemas"]["Coaster"];
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
  "/records/": {
    /**
     * TODO
     * @description TODO
     */
    get: operations["record.main"];
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
     * Login with username and password and receive a JWT
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
  };
  "/users/user/:user_id/": {
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
    Coaster: {
      /** Format: uint32 */
      height?: number | null;
      /** Format: uint32 */
      id: number;
      image?: string | null;
      /** Format: uint32 */
      inversions?: number | null;
      /** Format: uint32 */
      length?: number | null;
      manufacturer?: components["schemas"]["Manufacturer"] | null;
      name: string;
      park?: components["schemas"]["Park"] | null;
      /** Format: uint32 */
      speed?: number | null;
    };
    CoasterAndBucketListCount: {
      /** Format: uint32 */
      bucket_list_count: number;
      coaster: components["schemas"]["Coaster"];
    };
    /** @enum {string} */
    Country: "Argentina" | "Australia" | "Austria" | "Belgium" | "Brazil" | "Burma" | "Canada" | "China" | "Colombia" | "Cyprus" | "CzechRepublic" | "Denmark" | "Finland" | "France" | "Germany" | "Guatemala" | "Hungary" | "India" | "Indonesia" | "Iraq" | "Ireland" | "Israel" | "Italy" | "Japan" | "Lebanon" | "Malaysia" | "Mexico" | "Mongolia" | "Netherlands" | "NewZealand" | "Norway" | "Peru" | "Poland" | "Portugal" | "Qatar" | "Russia" | "Singapore" | "SouthAfrica" | "SouthKorea" | "Spain" | "Sweden" | "Switzerland" | "Taiwan" | "Thailand" | "Turkey" | "Ukraine" | "UnitedArabEmirates" | "UnitedKingdom" | "UnitedStates" | "Vietnam";
    Manufacturer: {
      name: string;
    };
    Park: {
      country?: components["schemas"]["Country"] | null;
      /** Format: uint32 */
      id: number;
      name: string;
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
      id: number;
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
   * TODO
   * @description TODO
   */
  "record.main": {
    responses: {
      200: {
        content: never;
      };
    };
  };
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
   * Login with username and password and receive a JWT
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
      400: {
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
